use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SoundEffect {
    KeyClick,
    KeyError,
    WordComplete,
    ExerciseSuccess,
    GamePop,
    GameBeamDrop,
    CountdownBeep,
}

use std::collections::{HashMap, VecDeque};
use std::sync::Condvar;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;
const BELOW_NORMAL_PRIORITY_CLASS: u32 = 0x00004000;

enum SpeechTask {
    Speak { text: String, interrupt: bool },
    Stop,
}

struct PiperConfig {
    bin_path: PathBuf,
    model_path: PathBuf,
}

impl PiperConfig {
    fn detect() -> Option<Self> {
        let bin_candidates = [
            PathBuf::from("assets/piper/bin/piper.exe"),
            PathBuf::from("assets/piper/bin/piper"),
            PathBuf::from("assets/piper/piper.exe"),
        ];
        let model_candidates = [
            PathBuf::from("assets/piper/models/en_US-amy-medium.onnx"),
        ];

        let bin = bin_candidates.into_iter().find(|p| p.exists())?;
        let model = model_candidates.into_iter().find(|p| p.exists())?;

        Some(Self {
            bin_path: bin,
            model_path: model,
        })
    }

    fn synthesize_raw(&self, text: &str) -> Option<Vec<f32>> {
        if text.trim().is_empty() {
            return None;
        }

        let mut cmd = Command::new(&self.bin_path);
        cmd.arg("--model")
            .arg(&self.model_path)
            .arg("--output_raw")
            .arg("--length_scale")
            .arg("0.92")
            .arg("--noise_scale")
            .arg("0.75")
            .arg("--noise_w")
            .arg("0.85")
            .arg("-q")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());

        #[cfg(windows)]
        cmd.creation_flags(CREATE_NO_WINDOW | BELOW_NORMAL_PRIORITY_CLASS);

        let mut child = cmd.spawn().ok()?;

        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
            let _ = stdin.write_all(b"\n");
        }

        let output = child.wait_with_output().ok()?;
        if !output.status.success() || output.stdout.is_empty() {
            return None;
        }

        let raw_bytes = output.stdout;
        let mut samples = Vec::with_capacity(raw_bytes.len() / 2);
        for chunk in raw_bytes.chunks_exact(2) {
            let s = i16::from_le_bytes([chunk[0], chunk[1]]);
            samples.push(s as f32 / 32768.0);
        }

        Some(samples)
    }
}

struct TtsWorker {
    task_sender: Sender<SpeechTask>,
    precache_queue: Arc<(Mutex<VecDeque<String>>, Condvar)>,
    sample_cache: Arc<Mutex<HashMap<String, Vec<f32>>>>,
}

impl TtsWorker {
    fn new(
        _stream_handle_opt: Option<OutputStreamHandle>,
        speech_sink: Arc<Mutex<Option<Sink>>>,
    ) -> Option<Self> {
        let (task_sender, task_receiver) = mpsc::channel::<SpeechTask>();
        let sample_cache = Arc::new(Mutex::new(HashMap::<String, Vec<f32>>::new()));
        let precache_queue = Arc::new((Mutex::new(VecDeque::<String>::new()), Condvar::new()));
        let piper_opt = Arc::new(PiperConfig::detect());

        // 1. Background Pre-caching Thread (Synthesizes words strictly in order of appearance)
        {
            let precache_queue = Arc::clone(&precache_queue);
            let sample_cache = Arc::clone(&sample_cache);
            let piper_opt = Arc::clone(&piper_opt);

            thread::Builder::new()
                .name("type-student-precache".into())
                .spawn(move || {
                    while let Some(piper) = piper_opt.as_ref() {
                        let word = {
                            let (lock, cvar) = &*precache_queue;
                            let mut queue = lock.lock().unwrap();
                            while queue.is_empty() {
                                queue = cvar.wait(queue).unwrap();
                            }
                            queue.pop_front()
                        };

                        if let Some(w) = word {
                            let key = w.trim().to_lowercase();
                            let already_cached = {
                                let cache = sample_cache.lock().unwrap();
                                cache.contains_key(&key)
                            };

                            if !key.is_empty() && !already_cached {
                                if let Some(samples) = piper.synthesize_raw(&w) {
                                    let mut cache = sample_cache.lock().unwrap();
                                    cache.insert(key, samples);
                                }
                                thread::sleep(std::time::Duration::from_millis(20));
                            }
                        }
                    }
                })
                .ok()?;
        }

        // 2. Playback & Instant Fallback Speech Worker Thread
        {
            let sample_cache = Arc::clone(&sample_cache);
            let speech_sink_clone = Arc::clone(&speech_sink);

            thread::Builder::new()
                .name("type-student-speech".into())
                .spawn(move || {
                    let mut os_tts = {
                        let mut t = tts::Tts::default().ok();
                        if let Some(tts_engine) = &mut t {
                            let _ = tts_engine.set_rate(0.95);
                            if let Ok(voices) = tts_engine.voices() {
                                let preferred_voice = voices.iter().find(|v| {
                                    let name = v.name().to_lowercase();
                                    name.contains("natural") || name.contains("jenny") || name.contains("aria")
                                }).or_else(|| {
                                    voices.iter().find(|v| {
                                        let name = v.name().to_lowercase();
                                        name.contains("zira") || name.contains("eva") || name.contains("mark") || name.contains("guy")
                                    })
                                }).or_else(|| {
                                    voices.iter().find(|v| {
                                        let lang = v.language().to_lowercase();
                                        lang.starts_with("en")
                                    })
                                });

                                if let Some(voice) = preferred_voice {
                                    let _ = tts_engine.set_voice(voice);
                                }
                            }
                        }
                        t
                    };

                    while let Ok(task) = task_receiver.recv() {
                        match task {
                            SpeechTask::Speak { text, interrupt } => {
                                let key = text.trim().to_lowercase();
                                // 1. Check if neural sample was pre-cached in memory
                                let cached_opt = {
                                    let cache = sample_cache.lock().unwrap();
                                    cache.get(&key).cloned()
                                };

                                if let Some(samples) = cached_opt {
                                    if let Ok(mut sink_guard) = speech_sink_clone.lock() {
                                        if let Some(sink) = sink_guard.as_mut() {
                                            if interrupt {
                                                sink.clear();
                                            }
                                            let source = rodio::buffer::SamplesBuffer::new(1, 22050, samples);
                                            sink.set_volume(0.95);
                                            sink.append(source);
                                        }
                                    }
                                } else if let Some(tts_engine) = &mut os_tts {
                                    // 2. Real-time in-process OS TTS fallback (0ms latency, zero process spawn!)
                                    let _ = tts_engine.speak(&text, interrupt);
                                }
                            }
                            SpeechTask::Stop => {
                                if let Ok(mut sink_guard) = speech_sink_clone.lock() {
                                    if let Some(sink) = sink_guard.as_mut() {
                                        sink.clear();
                                    }
                                }
                                if let Some(tts_engine) = &mut os_tts {
                                    let _ = tts_engine.stop();
                                }
                            }
                        }
                    }
                })
                .ok()?;
        }

        let worker = Self {
            task_sender,
            precache_queue,
            sample_cache,
        };

        // Pre-cache primary alphabet letters in background
        worker.precache(vec![
            "F".into(), "J".into(), "D".into(), "K".into(), "S".into(), "L".into(),
            "A".into(), "Space".into(), "Semicolon".into(), "G".into(), "H".into(),
            "E".into(), "I".into(), "R".into(), "U".into(), "T".into(), "Y".into(),
            "W".into(), "O".into(), "Q".into(), "P".into(), "C".into(), "V".into(),
            "B".into(), "N".into(), "M".into(), "Z".into(), "X".into(),
        ]);

        Some(worker)
    }

    fn precache(&self, words: Vec<String>) {
        let (lock, cvar) = &*self.precache_queue;
        let mut queue = lock.lock().unwrap();
        let mut new_queue = VecDeque::new();
        {
            let cache = self.sample_cache.lock().unwrap();
            for w in words {
                let key = w.trim().to_lowercase();
                if !key.is_empty() && !cache.contains_key(&key) && !new_queue.contains(&w) {
                    new_queue.push_back(w);
                }
            }
        }
        for old in queue.drain(..) {
            if !new_queue.contains(&old) {
                new_queue.push_back(old);
            }
        }
        *queue = new_queue;
        cvar.notify_all();
    }

    fn speak(&self, text: String, interrupt: bool) {
        // Asynchronously dispatch all speech to worker thread — zero main-thread blocking
        let _ = self.task_sender.send(SpeechTask::Speak { text, interrupt });
    }

    fn stop(&self) {
        let _ = self.task_sender.send(SpeechTask::Stop);
    }
}

pub fn char_speech_name(c: char) -> String {
    match c.to_ascii_lowercase() {
        'a'..='z' => c.to_ascii_uppercase().to_string(),
        '0'..='9' => c.to_string(),
        ' ' => "Space".to_string(),
        ';' => "Semicolon".to_string(),
        ':' => "Colon".to_string(),
        ',' => "Comma".to_string(),
        '.' => "Period".to_string(),
        '!' => "Exclamation".to_string(),
        '?' => "Question mark".to_string(),
        '\'' => "Apostrophe".to_string(),
        '"' => "Quote".to_string(),
        '-' => "Hyphen".to_string(),
        '/' => "Slash".to_string(),
        '\\' => "Backslash".to_string(),
        '\n' => "Enter".to_string(),
        _ => c.to_string(),
    }
}

pub struct SoundGenerator {
    pub sample_rate: u32,
    cached_effects: HashMap<SoundEffect, Vec<f32>>,
}

impl SoundGenerator {
    pub fn new() -> Self {
        let sample_rate = 44100;
        let mut cached_effects = HashMap::new();

        let effects = [
            SoundEffect::KeyClick,
            SoundEffect::KeyError,
            SoundEffect::WordComplete,
            SoundEffect::ExerciseSuccess,
            SoundEffect::GamePop,
            SoundEffect::GameBeamDrop,
            SoundEffect::CountdownBeep,
        ];

        for &eff in &effects {
            cached_effects.insert(eff, Self::compute_samples(sample_rate, eff));
        }

        Self {
            sample_rate,
            cached_effects,
        }
    }

    pub fn get_samples(&self, effect: SoundEffect) -> Option<&Vec<f32>> {
        self.cached_effects.get(&effect)
    }

    fn compute_samples(sample_rate: u32, effect: SoundEffect) -> Vec<f32> {
        let sr = sample_rate as f32;
        let mut samples = Vec::new();

        match effect {
            SoundEffect::KeyClick => {
                let dur = 0.015;
                let total = (dur * sr) as usize;
                for i in 0..total {
                    let t = i as f32 / sr;
                    let env = 1.0 - (i as f32 / total as f32);
                    let wave = (2.0 * std::f32::consts::PI * 1800.0 * t).sin() * 0.4
                        + (2.0 * std::f32::consts::PI * 800.0 * t).sin() * 0.3;
                    samples.push(wave * env * 0.5);
                }
            }
            SoundEffect::KeyError => {
                let dur = 0.095;
                let total = (dur * sr) as usize;
                for i in 0..total {
                    let t = i as f32 / sr;
                    let env = (1.0 - (i as f32 / total as f32)).powf(1.2);
                    let wave = (2.0 * std::f32::consts::PI * 130.0 * t).sin() * 0.7
                        + (2.0 * std::f32::consts::PI * 65.0 * t).sin() * 0.4
                        + (2.0 * std::f32::consts::PI * 260.0 * t).sin() * 0.2;
                    samples.push(wave * env * 0.85);
                }
            }
            SoundEffect::WordComplete => {
                let dur = 0.08;
                let total = (dur * sr) as usize;
                for i in 0..total {
                    let t = i as f32 / sr;
                    let env = 1.0 - (i as f32 / total as f32);
                    let wave = (2.0 * std::f32::consts::PI * 880.0 * t).sin();
                    samples.push(wave * env * 0.35);
                }
            }
            SoundEffect::ExerciseSuccess => {
                let tones = [523.25, 659.25, 783.99, 1046.50];
                let tone_dur = 0.10;
                for &freq in &tones {
                    let total = (tone_dur * sr) as usize;
                    for i in 0..total {
                        let t = i as f32 / sr;
                        let env = (1.0 - (i as f32 / total as f32)).powf(0.8);
                        let wave = (2.0 * std::f32::consts::PI * freq * t).sin() * 0.7
                            + (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin() * 0.2;
                        samples.push(wave * env * 0.45);
                    }
                }
            }
            SoundEffect::GamePop => {
                let dur = 0.07;
                let total = (dur * sr) as usize;
                for i in 0..total {
                    let frac = i as f32 / total as f32;
                    let t = i as f32 / sr;
                    let freq = 400.0 + frac * 800.0;
                    let env = (1.0 - frac).powf(1.5);
                    let wave = (2.0 * std::f32::consts::PI * freq * t).sin();
                    samples.push(wave * env * 0.5);
                }
            }
            SoundEffect::GameBeamDrop => {
                let dur = 0.05;
                let total = (dur * sr) as usize;
                for i in 0..total {
                    let t = i as f32 / sr;
                    let env = (1.0 - (i as f32 / total as f32)).powf(2.0);
                    let wave = (2.0 * std::f32::consts::PI * 220.0 * t).sin() * 0.8;
                    samples.push(wave * env * 0.5);
                }
            }
            SoundEffect::CountdownBeep => {
                let dur = 0.06;
                let total = (dur * sr) as usize;
                for i in 0..total {
                    let t = i as f32 / sr;
                    let env = 1.0 - (i as f32 / total as f32);
                    let wave = (2.0 * std::f32::consts::PI * 660.0 * t).sin();
                    samples.push(wave * env * 0.4);
                }
            }
        }
        samples
    }
}

pub struct AudioEngine {
    _stream: Option<OutputStream>,
    sfx_sink: Arc<Mutex<Option<Sink>>>,
    generator: SoundGenerator,
    tts: Option<TtsWorker>,
    enabled: bool,
    voice_enabled: bool,
    volume: f32,
}

impl AudioEngine {
    pub fn new() -> Self {
        let (stream, stream_handle) = match OutputStream::try_default() {
            Ok((s, h)) => (Some(s), Some(h)),
            Err(_) => (None, None),
        };

        let sfx_sink = Arc::new(Mutex::new(
            stream_handle.as_ref().and_then(|h| Sink::try_new(h).ok()),
        ));
        let speech_sink = Arc::new(Mutex::new(
            stream_handle.as_ref().and_then(|h| Sink::try_new(h).ok()),
        ));

        let tts = TtsWorker::new(stream_handle, Arc::clone(&speech_sink));

        Self {
            _stream: stream,
            sfx_sink,
            generator: SoundGenerator::new(),
            tts,
            enabled: true,
            voice_enabled: true,
            volume: 0.7,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_voice_enabled(&mut self, voice_enabled: bool) {
        self.voice_enabled = voice_enabled;
        if !voice_enabled {
            self.stop_speech();
        }
    }

    pub fn is_voice_enabled(&self) -> bool {
        self.voice_enabled
    }

    pub fn toggle_voice(&mut self) -> bool {
        self.voice_enabled = !self.voice_enabled;
        if !self.voice_enabled {
            self.stop_speech();
        }
        self.voice_enabled
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn play(&self, effect: SoundEffect) {
        if !self.enabled || self.volume <= 0.0 {
            return;
        }

        if let Ok(mut sink_guard) = self.sfx_sink.try_lock() {
            if let Some(sink) = sink_guard.as_mut() {
                if let Some(samples) = self.generator.get_samples(effect) {
                    let source = rodio::buffer::SamplesBuffer::new(1, self.generator.sample_rate, samples.clone());
                    sink.set_volume(self.volume);
                    sink.append(source);
                }
            }
        }
    }

    pub fn speak_letter(&self, c: char) {
        if !self.enabled || !self.voice_enabled {
            return;
        }
        if let Some(tts) = &self.tts {
            let name = char_speech_name(c);
            tts.speak(name, true);
        }
    }

    pub fn speak_word(&self, word: &str) {
        if !self.enabled || !self.voice_enabled || word.is_empty() {
            return;
        }
        if let Some(tts) = &self.tts {
            tts.speak(word.to_string(), true);
        }
    }

    pub fn speak_text(&self, text: &str, interrupt: bool) {
        if !self.enabled || !self.voice_enabled || text.is_empty() {
            return;
        }
        if let Some(tts) = &self.tts {
            tts.speak(text.to_string(), interrupt);
        }
    }

    pub fn precache_text(&self, text: &str) {
        if !self.enabled || !self.voice_enabled || text.is_empty() {
            return;
        }
        let mut words: Vec<String> = Vec::new();
        // 1. Process first 25 words in exact order of appearance to avoid overloading the CPU
        for word in text.split_whitespace() {
            let clean = word.trim_matches(|c: char| !c.is_alphanumeric());
            if !clean.is_empty() && !words.contains(&clean.to_string()) {
                words.push(clean.to_string());
                if words.len() >= 25 {
                    break;
                }
            }
        }
        // 2. Also register individual letters in order of appearance
        for ch in text.chars() {
            if ch.is_alphanumeric() || ch == ';' || ch == ':' || ch == ',' || ch == '.' {
                let name = char_speech_name(ch);
                if !words.contains(&name) {
                    words.push(name);
                }
            }
        }

        if let Some(tts) = &self.tts {
            tts.precache(words);
        }
    }

    pub fn stop_speech(&self) {
        if let Some(tts) = &self.tts {
            tts.stop();
        }
    }
}

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

enum AudioMsg {
    Play(SoundEffect),
    SpeakLetter(char),
    SpeakWord(String),
    SpeakText(String, bool),
    PrecacheText(String),
    StopSpeech,
    SetVolume(f32),
    SetEnabled(bool),
    SetVoiceEnabled(bool),
}

#[derive(Clone)]
pub struct SharedAudio {
    sender: Sender<AudioMsg>,
    enabled: Arc<AtomicBool>,
    voice_enabled: Arc<AtomicBool>,
    volume_bits: Arc<AtomicU32>,
}

impl SharedAudio {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel::<AudioMsg>();
        let enabled = Arc::new(AtomicBool::new(true));
        let voice_enabled = Arc::new(AtomicBool::new(true));
        let volume_bits = Arc::new(AtomicU32::new(0.7f32.to_bits()));

        // Spawn dedicated Audio Actor Thread that manages the audio hardware & sinks
        thread::Builder::new()
            .name("type-student-audio-actor".into())
            .spawn(move || {
                let mut engine = AudioEngine::new();
                while let Ok(msg) = receiver.recv() {
                    match msg {
                        AudioMsg::Play(effect) => engine.play(effect),
                        AudioMsg::SpeakLetter(c) => engine.speak_letter(c),
                        AudioMsg::SpeakWord(w) => engine.speak_word(&w),
                        AudioMsg::SpeakText(text, interrupt) => engine.speak_text(&text, interrupt),
                        AudioMsg::PrecacheText(text) => engine.precache_text(&text),
                        AudioMsg::StopSpeech => engine.stop_speech(),
                        AudioMsg::SetVolume(vol) => engine.set_volume(vol),
                        AudioMsg::SetEnabled(en) => engine.set_enabled(en),
                        AudioMsg::SetVoiceEnabled(v_en) => engine.set_voice_enabled(v_en),
                    }
                }
            })
            .expect("failed to spawn audio actor thread");

        Self {
            sender,
            enabled,
            voice_enabled,
            volume_bits,
        }
    }

    #[inline]
    pub fn play(&self, effect: SoundEffect) {
        if self.enabled.load(Ordering::Relaxed) {
            let _ = self.sender.send(AudioMsg::Play(effect));
        }
    }

    #[inline]
    pub fn precache_text(&self, text: &str) {
        if self.enabled.load(Ordering::Relaxed) && self.voice_enabled.load(Ordering::Relaxed) {
            let _ = self.sender.send(AudioMsg::PrecacheText(text.to_string()));
        }
    }

    #[inline]
    pub fn speak_letter(&self, c: char) {
        if self.enabled.load(Ordering::Relaxed) && self.voice_enabled.load(Ordering::Relaxed) {
            let _ = self.sender.send(AudioMsg::SpeakLetter(c));
        }
    }

    #[inline]
    pub fn speak_word(&self, word: &str) {
        if self.enabled.load(Ordering::Relaxed) && self.voice_enabled.load(Ordering::Relaxed) && !word.is_empty() {
            let _ = self.sender.send(AudioMsg::SpeakWord(word.to_string()));
        }
    }

    #[inline]
    pub fn speak_text(&self, text: &str, interrupt: bool) {
        if self.enabled.load(Ordering::Relaxed) && self.voice_enabled.load(Ordering::Relaxed) && !text.is_empty() {
            let _ = self.sender.send(AudioMsg::SpeakText(text.to_string(), interrupt));
        }
    }

    #[inline]
    pub fn stop_speech(&self) {
        let _ = self.sender.send(AudioMsg::StopSpeech);
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
        let _ = self.sender.send(AudioMsg::SetEnabled(enabled));
    }

    pub fn set_voice_enabled(&self, enabled: bool) {
        self.voice_enabled.store(enabled, Ordering::Relaxed);
        let _ = self.sender.send(AudioMsg::SetVoiceEnabled(enabled));
    }

    #[inline]
    pub fn is_voice_enabled(&self) -> bool {
        self.voice_enabled.load(Ordering::Relaxed)
    }

    pub fn toggle_voice(&self) -> bool {
        let current = self.voice_enabled.load(Ordering::Relaxed);
        let new_state = !current;
        self.voice_enabled.store(new_state, Ordering::Relaxed);
        let _ = self.sender.send(AudioMsg::SetVoiceEnabled(new_state));
        new_state
    }

    pub fn set_volume(&self, volume: f32) {
        self.volume_bits.store(volume.to_bits(), Ordering::Relaxed);
        let _ = self.sender.send(AudioMsg::SetVolume(volume));
    }
}

