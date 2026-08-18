use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoundEffect {
    KeyClick,
    KeyError,
    WordComplete,
    ExerciseSuccess,
    GamePop,
    GameBeamDrop,
    CountdownBeep,
}

pub struct SoundGenerator {
    sample_rate: u32,
}

impl SoundGenerator {
    pub fn new() -> Self {
        Self { sample_rate: 44100 }
    }

    pub fn generate_samples(&self, effect: SoundEffect) -> Vec<f32> {
        let sr = self.sample_rate as f32;
        let mut samples = Vec::new();

        match effect {
            SoundEffect::KeyClick => {
                // Short crisp click (15ms)
                let dur = 0.015;
                let total = (dur * sr) as usize;
                for i in 0..total {
                    let t = i as f32 / sr;
                    let env = 1.0 - (i as f32 / total as f32);
                    // High frequency burst mixed with short pop
                    let wave = (2.0 * std::f32::consts::PI * 1800.0 * t).sin() * 0.4
                        + (2.0 * std::f32::consts::PI * 800.0 * t).sin() * 0.3;
                    samples.push(wave * env * 0.5);
                }
            }
            SoundEffect::KeyError => {
                // Low thud / buzz (90ms)
                let dur = 0.090;
                let total = (dur * sr) as usize;
                for i in 0..total {
                    let t = i as f32 / sr;
                    let env = 1.0 - (i as f32 / total as f32);
                    let wave = (2.0 * std::f32::consts::PI * 140.0 * t).sin() * 0.6
                        + (2.0 * std::f32::consts::PI * 70.0 * t).sin() * 0.4;
                    samples.push(wave * env * 0.6);
                }
            }
            SoundEffect::WordComplete => {
                // Gentle chime (80ms)
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
                // Ascending 3-tone arpeggio (C5 -> E5 -> G5 -> C6)
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
                // Upward pitch-sweep bubble pop
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
                // Mechanical heavy drop click
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
                // Clean short beep (60ms, 660Hz)
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
    stream_handle: Option<OutputStreamHandle>,
    generator: SoundGenerator,
    enabled: bool,
    volume: f32,
}

impl AudioEngine {
    pub fn new() -> Self {
        let (stream, stream_handle) = match OutputStream::try_default() {
            Ok((s, h)) => (Some(s), Some(h)),
            Err(_) => (None, None),
        };

        Self {
            _stream: stream,
            stream_handle,
            generator: SoundGenerator::new(),
            enabled: true,
            volume: 0.7,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn play(&self, effect: SoundEffect) {
        if !self.enabled || self.volume <= 0.0 {
            return;
        }

        if let Some(handle) = &self.stream_handle {
            let samples = self.generator.generate_samples(effect);
            let volume = self.volume;
            let sample_rate = self.generator.sample_rate;

            let source = rodio::buffer::SamplesBuffer::new(1, sample_rate, samples);
            if let Ok(sink) = Sink::try_new(handle) {
                sink.set_volume(volume);
                sink.append(source);
                sink.detach();
            }
        }
    }
}

#[derive(Clone)]
pub struct SharedAudio(Arc<Mutex<AudioEngine>>);

impl SharedAudio {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(AudioEngine::new())))
    }

    pub fn play(&self, effect: SoundEffect) {
        if let Ok(audio) = self.0.lock() {
            audio.play(effect);
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        if let Ok(mut audio) = self.0.lock() {
            audio.set_enabled(enabled);
        }
    }

    pub fn set_volume(&self, volume: f32) {
        if let Ok(mut audio) = self.0.lock() {
            audio.set_volume(volume);
        }
    }
}
