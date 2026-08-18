use type_student::components::keyboard::{get_finger_for_char, Finger};
use type_student::course::get_all_lessons;
use type_student::engine::{CharStatus, TypingSession};
use type_student::profile::UserProfile;
use type_student::views::games::{AbcGame, BubblesGame, CloudsGame, WordTrisGame};

#[test]
fn test_lessons_structure() {
    let lessons = get_all_lessons();
    assert_eq!(lessons.len(), 12, "Should have 12 touch typing lessons");
    
    // Check Lesson 1 Home Row
    assert_eq!(lessons[0].number, 1);
    assert_eq!(lessons[0].keys_introduced, vec!['a', 's', 'd', 'f', 'j', 'k', 'l', ';']);
    assert!(!lessons[0].exercises.is_empty());
}

#[test]
fn test_typing_session_metrics() {
    let mut session = TypingSession::new("hello world");
    assert_eq!(session.current_char(), Some('h'));
    assert_eq!(session.next_char(), Some('e'));
    
    // Type correct chars
    assert!(session.handle_char_input('h'));
    assert_eq!(session.char_statuses[0], CharStatus::Correct);
    assert_eq!(session.cursor_idx, 1);
    
    // Type incorrect char
    assert!(!session.handle_char_input('x')); // expected 'e'
    assert_eq!(session.char_statuses[1], CharStatus::Incorrect('x'));
    
    // Backspace
    assert!(session.handle_backspace());
    assert_eq!(session.cursor_idx, 1);
    assert_eq!(session.char_statuses[1], CharStatus::Pending);
    
    // Retype correctly
    assert!(session.handle_char_input('e'));
    assert_eq!(session.char_statuses[1], CharStatus::Correct);
}

#[test]
fn test_finger_mapping() {
    assert_eq!(get_finger_for_char('a'), Finger::LeftPinky);
    assert_eq!(get_finger_for_char('f'), Finger::LeftIndex);
    assert_eq!(get_finger_for_char('j'), Finger::RightIndex);
    assert_eq!(get_finger_for_char('k'), Finger::RightMiddle);
    assert_eq!(get_finger_for_char('l'), Finger::RightRing);
    assert_eq!(get_finger_for_char(';'), Finger::RightPinky);
    assert_eq!(get_finger_for_char(' '), Finger::Thumb);
}

#[test]
fn test_bubbles_game() {
    let mut game = BubblesGame::new();
    assert_eq!(game.lives, 5);
    assert_eq!(game.score, 0);
    
    game.spawn_bubble();
    assert_eq!(game.bubbles.len(), 1);
    
    let bubble_text = game.bubbles[0].text.clone();
    let first_char = bubble_text.chars().next().unwrap();
    
    let (matched, _) = game.handle_char(first_char);
    assert!(matched, "Should match first character of spawned bubble");
}

#[test]
fn test_wordtris_game() {
    let mut game = WordTrisGame::new();
    game.spawn_beam();
    assert_eq!(game.active_beams.len(), 1);
    
    let beam_text = game.active_beams[0].text.clone();
    let first_char = beam_text.chars().next().unwrap();
    assert!(game.handle_char(first_char));
}

#[test]
fn test_clouds_game() {
    let mut game = CloudsGame::new();
    game.spawn_cloud();
    assert_eq!(game.clouds.len(), 1);
    let cloud_text = game.clouds[0].text.clone();
    
    // Type characters
    for c in cloud_text.chars() {
        game.handle_char(c);
    }
    // Press space to clear
    assert!(game.handle_char(' '));
    assert!(game.score > 0);
}

#[test]
fn test_abc_game() {
    let mut game = AbcGame::new();
    assert_eq!(game.current_char(), Some('a'));
    
    let (correct, finished) = game.handle_char('a');
    assert!(correct);
    assert!(!finished);
    assert_eq!(game.current_char(), Some('b'));
}

#[test]
fn test_profile_difficult_keys() {
    let mut profile = UserProfile::new("test_user".to_string(), "Test User".to_string(), "🌟".to_string());
    
    // Record good stats for 'a'
    for _ in 0..10 {
        profile.record_keystroke('a', true, 120);
    }
    
    // Record poor stats for 'q'
    for _ in 0..2 {
        profile.record_keystroke('q', true, 300);
    }
    for _ in 0..6 {
        profile.record_keystroke('q', false, 350);
    }
    
    let difficult = profile.get_difficult_keys();
    assert!(!difficult.is_empty());
    assert_eq!(difficult[0].0, 'q');
}

#[test]
fn test_trophies_and_stories() {
    use type_student::trophies::get_all_trophies;
    use type_student::views::stories_data::get_all_stories;
    use type_student::views::diploma_export::export_diploma_html;

    let trophies = get_all_trophies();
    assert_eq!(trophies.len(), 12, "Should have 12 trophies");

    let stories = get_all_stories();
    assert_eq!(stories.len(), 6, "Should have 6 fun story presets");

    let path = export_diploma_html("Alice", "Aesop's Fables", 45.0, 98.5, "August 18, 2026", "2 Minutes");
    assert!(path.is_some());
    let p = path.unwrap();
    assert!(p.exists());
    let _ = std::fs::remove_file(p);
}

