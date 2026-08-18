use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExerciseType {
    KeyIntro,
    KeyDrill,
    WordDrill,
    SentenceDrill,
    ParagraphDrill,
    LessonTest,
}

impl ExerciseType {
    pub fn label(&self) -> &'static str {
        match self {
            ExerciseType::KeyIntro => "Key Introduction",
            ExerciseType::KeyDrill => "Key Drill",
            ExerciseType::WordDrill => "Word Drill",
            ExerciseType::SentenceDrill => "Sentence Drill",
            ExerciseType::ParagraphDrill => "Paragraph Drill",
            ExerciseType::LessonTest => "Lesson Test",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ExerciseType::KeyIntro => "💡",
            ExerciseType::KeyDrill => "⌨️",
            ExerciseType::WordDrill => "📖",
            ExerciseType::SentenceDrill => "📝",
            ExerciseType::ParagraphDrill => "📜",
            ExerciseType::LessonTest => "🏆",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Exercise {
    pub id: String,
    pub title: String,
    pub exercise_type: ExerciseType,
    pub instruction: String,
    pub new_keys: Vec<char>,
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lesson {
    pub id: String,
    pub number: u8,
    pub title: String,
    pub subtitle: String,
    pub keys_introduced: Vec<char>,
    pub exercises: Vec<Exercise>,
}

pub fn get_all_lessons() -> Vec<Lesson> {
    vec![
        // Lesson 1: Home Row (A, S, D, F, J, K, L, ;)
        Lesson {
            id: "lesson_1".to_string(),
            number: 1,
            title: "The Home Row".to_string(),
            subtitle: "A S D F and J K L ; - The foundation of touch typing".to_string(),
            keys_introduced: vec!['a', 's', 'd', 'f', 'j', 'k', 'l', ';'],
            exercises: vec![
                Exercise {
                    id: "1_1".to_string(),
                    title: "Home Row Keys".to_string(),
                    exercise_type: ExerciseType::KeyIntro,
                    instruction: "Rest your left fingers on A S D F and right fingers on J K L ;. Feel the bumps on F and J.".to_string(),
                    new_keys: vec!['a', 's', 'd', 'f', 'j', 'k', 'l', ';'],
                    text: "asdf jkl; asdf jkl; asdf jkl; asdf jkl;".to_string(),
                },
                Exercise {
                    id: "1_2".to_string(),
                    title: "Left Hand Focus".to_string(),
                    exercise_type: ExerciseType::KeyDrill,
                    instruction: "Keep your left fingers light and relaxed on the home row keys.".to_string(),
                    new_keys: vec!['a', 's', 'd', 'f'],
                    text: "aaa sss ddd fff asdf fdda sada fads faas dafd".to_string(),
                },
                Exercise {
                    id: "1_3".to_string(),
                    title: "Right Hand Focus".to_string(),
                    exercise_type: ExerciseType::KeyDrill,
                    instruction: "Keep your right fingers relaxed on the home row keys.".to_string(),
                    new_keys: vec!['j', 'k', 'l', ';'],
                    text: "jjj kkk lll ;;; jkl; ;;ll kkjj lk;j j;kl ljk;".to_string(),
                },
                Exercise {
                    id: "1_4".to_string(),
                    title: "Home Row Words".to_string(),
                    exercise_type: ExerciseType::WordDrill,
                    instruction: "Type these home row words smoothly without looking at your hands.".to_string(),
                    new_keys: vec![],
                    text: "as ask dad fad lad fall flask salad flash lass fall add fad".to_string(),
                },
                Exercise {
                    id: "1_5".to_string(),
                    title: "Home Row Sentences".to_string(),
                    exercise_type: ExerciseType::SentenceDrill,
                    instruction: "Maintain a steady typing rhythm across the home row.".to_string(),
                    new_keys: vec![],
                    text: "a lad asks dad; all lads ask dad; dad had a salad; flash fall;".to_string(),
                },
                Exercise {
                    id: "1_6".to_string(),
                    title: "Lesson 1 Review Test".to_string(),
                    exercise_type: ExerciseType::LessonTest,
                    instruction: "Test your speed and accuracy on the entire home row!".to_string(),
                    new_keys: vec![],
                    text: "dad had a salad; a lad had a flask; ask dad as a lad falls; flask and salad;".to_string(),
                },
            ],
        },

        // Lesson 2: Keys E and I
        Lesson {
            id: "lesson_2".to_string(),
            number: 2,
            title: "Keys E and I".to_string(),
            subtitle: "Upper row reach with your middle fingers".to_string(),
            keys_introduced: vec!['e', 'i'],
            exercises: vec![
                Exercise {
                    id: "2_1".to_string(),
                    title: "Key Intro: E and I".to_string(),
                    exercise_type: ExerciseType::KeyIntro,
                    instruction: "Reach up with your left middle finger to E, and right middle finger to I.".to_string(),
                    new_keys: vec!['e', 'i'],
                    text: "ded kik ded kik ded kik eee iii de ki de ki".to_string(),
                },
                Exercise {
                    id: "2_2".to_string(),
                    title: "E and I Drills".to_string(),
                    exercise_type: ExerciseType::KeyDrill,
                    instruction: "Always return your fingers to D and K after striking E and I.".to_string(),
                    new_keys: vec!['e', 'i'],
                    text: "die kid led fed file life idle seek feed disk side skill dill".to_string(),
                },
                Exercise {
                    id: "2_3".to_string(),
                    title: "Word Drill: E and I".to_string(),
                    exercise_type: ExerciseType::WordDrill,
                    instruction: "Build full words using the home row plus E and I.".to_string(),
                    new_keys: vec!['e', 'i'],
                    text: "deal feel silk leaf alike safe fail file field ideal slide dial".to_string(),
                },
                Exercise {
                    id: "2_4".to_string(),
                    title: "Sentence Practice".to_string(),
                    exercise_type: ExerciseType::SentenceDrill,
                    instruction: "Type full sentences with natural spacing.".to_string(),
                    new_keys: vec![],
                    text: "a silk dress is safe; feel free to slide; fill a desk with files;".to_string(),
                },
                Exercise {
                    id: "2_5".to_string(),
                    title: "Lesson 2 Test".to_string(),
                    exercise_type: ExerciseType::LessonTest,
                    instruction: "Comprehensive test for Home Row and keys E and I.".to_string(),
                    new_keys: vec![],
                    text: "a skillful kid feels safe; alike lads seek ideal files; fill the desk daily;".to_string(),
                },
            ],
        },

        // Lesson 3: Keys R and U
        Lesson {
            id: "lesson_3".to_string(),
            number: 3,
            title: "Keys R and U".to_string(),
            subtitle: "Upper row reach with your index fingers".to_string(),
            keys_introduced: vec!['r', 'u'],
            exercises: vec![
                Exercise {
                    id: "3_1".to_string(),
                    title: "Key Intro: R and U".to_string(),
                    exercise_type: ExerciseType::KeyIntro,
                    instruction: "Left index finger reaches up to R; right index finger reaches up to U.".to_string(),
                    new_keys: vec!['r', 'u'],
                    text: "frf juj frf juj rrr uuu fur ruf red use run rut".to_string(),
                },
                Exercise {
                    id: "3_2".to_string(),
                    title: "R and U Drills".to_string(),
                    exercise_type: ExerciseType::KeyDrill,
                    instruction: "Practice reaching cleanly to R and U without moving your whole wrist.".to_string(),
                    new_keys: vec!['r', 'u'],
                    text: "rule rust user surf sure risk ride dark farm hard sure true".to_string(),
                },
                Exercise {
                    id: "3_3".to_string(),
                    title: "Word Drill: R and U".to_string(),
                    exercise_type: ExerciseType::WordDrill,
                    instruction: "Type quickly and accurately as words become more varied.".to_string(),
                    new_keys: vec![],
                    text: "under ruler juror fluid fruit fraud curve earth dress guard sugar".to_string(),
                },
                Exercise {
                    id: "3_4".to_string(),
                    title: "Sentence Practice".to_string(),
                    exercise_type: ExerciseType::SentenceDrill,
                    instruction: "Keep your eyes on the screen, not your hands!".to_string(),
                    new_keys: vec![],
                    text: "the red ruler is useful; pure fruit juice is fresh; rural roads are hard;".to_string(),
                },
                Exercise {
                    id: "3_5".to_string(),
                    title: "Lesson 3 Test".to_string(),
                    exercise_type: ExerciseType::LessonTest,
                    instruction: "Demonstrate your speed with R, U, and previously learned keys.".to_string(),
                    new_keys: vec![],
                    text: "true friends share full fruit salads; a secure user guards fresh ideas;".to_string(),
                },
            ],
        },

        // Lesson 4: Keys T and O
        Lesson {
            id: "lesson_4".to_string(),
            number: 4,
            title: "Keys T and O".to_string(),
            subtitle: "Reaching T with index and O with ring finger".to_string(),
            keys_introduced: vec!['t', 'o'],
            exercises: vec![
                Exercise {
                    id: "4_1".to_string(),
                    title: "Key Intro: T and O".to_string(),
                    exercise_type: ExerciseType::KeyIntro,
                    instruction: "Left index reaches over to T; right ring finger reaches up to O.".to_string(),
                    new_keys: vec!['t', 'o'],
                    text: "ftf lol ftf lol ttt ooo tot lot out toe top old".to_string(),
                },
                Exercise {
                    id: "4_2".to_string(),
                    title: "T and O Drills".to_string(),
                    exercise_type: ExerciseType::KeyDrill,
                    instruction: "Smooth transitions between T and O.".to_string(),
                    new_keys: vec!['t', 'o'],
                    text: "tool told soft root look took foot stool door roof root cool".to_string(),
                },
                Exercise {
                    id: "4_3".to_string(),
                    title: "Vocabulary Expansion".to_string(),
                    exercise_type: ExerciseType::WordDrill,
                    instruction: "Common words featuring T and O.".to_string(),
                    new_keys: vec![],
                    text: "total trust outer toast rotor floor little turtle effort forgot".to_string(),
                },
                Exercise {
                    id: "4_4".to_string(),
                    title: "Sentences with T and O".to_string(),
                    exercise_type: ExerciseType::SentenceDrill,
                    instruction: "Type at a steady, rhythmic pace.".to_string(),
                    new_keys: vec![],
                    text: "look at the old oak tree; the little turtle took a rest on the forest floor;".to_string(),
                },
                Exercise {
                    id: "4_5".to_string(),
                    title: "Lesson 4 Test".to_string(),
                    exercise_type: ExerciseType::LessonTest,
                    instruction: "Test your mastery of T and O in combined context.".to_string(),
                    new_keys: vec![],
                    text: "soft outer wool feels good; the old tool door is locked for total safety;".to_string(),
                },
            ],
        },

        // Lesson 5: Keys C and Comma
        Lesson {
            id: "lesson_5".to_string(),
            number: 5,
            title: "Keys C and Comma".to_string(),
            subtitle: "Lower row reach with middle fingers".to_string(),
            keys_introduced: vec!['c', ','],
            exercises: vec![
                Exercise {
                    id: "5_1".to_string(),
                    title: "Key Intro: C and ,".to_string(),
                    exercise_type: ExerciseType::KeyIntro,
                    instruction: "Left middle reaches down to C; right middle reaches down to comma (,).".to_string(),
                    new_keys: vec!['c', ','],
                    text: "dcd k,k dcd k,k ccc ,,, cat cod car act ice ace".to_string(),
                },
                Exercise {
                    id: "5_2".to_string(),
                    title: "C and Comma Drills".to_string(),
                    exercise_type: ExerciseType::KeyDrill,
                    instruction: "Practice reaching down and curling fingers comfortably.".to_string(),
                    new_keys: vec!['c', ','],
                    text: "cake, cool, call, acid, calm, duck, track, fact, clock, clear,".to_string(),
                },
                Exercise {
                    id: "5_3".to_string(),
                    title: "Comma in Lists".to_string(),
                    exercise_type: ExerciseType::SentenceDrill,
                    instruction: "Type commas followed by a space.".to_string(),
                    new_keys: vec![],
                    text: "cats, ducks, deer, cattle, and raccoons all circle the cool creek;".to_string(),
                },
                Exercise {
                    id: "5_4".to_string(),
                    title: "Lesson 5 Test".to_string(),
                    exercise_type: ExerciseType::LessonTest,
                    instruction: "Evaluation on C and punctuation commas.".to_string(),
                    new_keys: vec![],
                    text: "clear, calm, and correct clicks create classic code, clear to see;".to_string(),
                },
            ],
        },

        // Lesson 6: Keys V and M
        Lesson {
            id: "lesson_6".to_string(),
            number: 6,
            title: "Keys V and M".to_string(),
            subtitle: "Lower row reach with index fingers".to_string(),
            keys_introduced: vec!['v', 'm'],
            exercises: vec![
                Exercise {
                    id: "6_1".to_string(),
                    title: "Key Intro: V and M".to_string(),
                    exercise_type: ExerciseType::KeyIntro,
                    instruction: "Left index reaches down to V; right index reaches down to M.".to_string(),
                    new_keys: vec!['v', 'm'],
                    text: "fvf jmj fvf jmj vvv mmm vim van men mom view mock".to_string(),
                },
                Exercise {
                    id: "6_2".to_string(),
                    title: "V and M Drills".to_string(),
                    exercise_type: ExerciseType::KeyDrill,
                    instruction: "Return index fingers immediately to F and J.".to_string(),
                    new_keys: vec!['v', 'm'],
                    text: "move, view, milk, vast, dive, memo, room, mark, cover, movie, metal".to_string(),
                },
                Exercise {
                    id: "6_3".to_string(),
                    title: "Sentences with V and M".to_string(),
                    exercise_type: ExerciseType::SentenceDrill,
                    instruction: "Full flowing sentences with V and M.".to_string(),
                    new_keys: vec![],
                    text: "music moves most men, making memories live forever; marvelous views;".to_string(),
                },
                Exercise {
                    id: "6_4".to_string(),
                    title: "Lesson 6 Test".to_string(),
                    exercise_type: ExerciseType::LessonTest,
                    instruction: "Test accuracy and speed for V and M.".to_string(),
                    new_keys: vec![],
                    text: "brave moves make marvelous music, and vast visual dreams motivate me;".to_string(),
                },
            ],
        },

        // Lesson 7: Keys B and N
        Lesson {
            id: "lesson_7".to_string(),
            number: 7,
            title: "Keys B and N".to_string(),
            subtitle: "Inner diagonal reach with index fingers".to_string(),
            keys_introduced: vec!['b', 'n'],
            exercises: vec![
                Exercise {
                    id: "7_1".to_string(),
                    title: "Key Intro: B and N".to_string(),
                    exercise_type: ExerciseType::KeyIntro,
                    instruction: "Left index extends to B; right index extends to N.".to_string(),
                    new_keys: vec!['b', 'n'],
                    text: "fbf jnj fbf jnj bbb nnn ban bin bob new not nab".to_string(),
                },
                Exercise {
                    id: "7_2".to_string(),
                    title: "B and N Drills".to_string(),
                    exercise_type: ExerciseType::KeyDrill,
                    instruction: "Stretching inward smoothly.".to_string(),
                    new_keys: vec!['b', 'n'],
                    text: "bone, bank, burn, band, noble, blend, brain, brown, number, balance".to_string(),
                },
                Exercise {
                    id: "7_3".to_string(),
                    title: "Sentences with B and N".to_string(),
                    exercise_type: ExerciseType::SentenceDrill,
                    instruction: "Connect words smoothly.".to_string(),
                    new_keys: vec![],
                    text: "birds build nests in brown branches, and brave bees bring bonus pollen;".to_string(),
                },
                Exercise {
                    id: "7_4".to_string(),
                    title: "Lesson 7 Test".to_string(),
                    exercise_type: ExerciseType::LessonTest,
                    instruction: "Test on B, N and previous keys.".to_string(),
                    new_keys: vec![],
                    text: "bright morning sunlight brings noble minds new boldness and balance;".to_string(),
                },
            ],
        },

        // Lesson 8: Keys W and P
        Lesson {
            id: "lesson_8".to_string(),
            number: 8,
            title: "Keys W and P".to_string(),
            subtitle: "Upper corner reach with ring and pinky fingers".to_string(),
            keys_introduced: vec!['w', 'p'],
            exercises: vec![
                Exercise {
                    id: "8_1".to_string(),
                    title: "Key Intro: W and P".to_string(),
                    exercise_type: ExerciseType::KeyIntro,
                    instruction: "Left ring finger reaches up to W; right pinky reaches up to P.".to_string(),
                    new_keys: vec!['w', 'p'],
                    text: "sws ;p; sws ;p; www ppp paw win wet pop web pen".to_string(),
                },
                Exercise {
                    id: "8_2".to_string(),
                    title: "W and P Drills".to_string(),
                    exercise_type: ExerciseType::KeyDrill,
                    instruction: "Strengthen pinky and ring finger independence.".to_string(),
                    new_keys: vec!['w', 'p'],
                    text: "power, paper, sweet, water, plant, planet, window, wrapper, purpose".to_string(),
                },
                Exercise {
                    id: "8_3".to_string(),
                    title: "Word & Sentence Practice".to_string(),
                    exercise_type: ExerciseType::SentenceDrill,
                    instruction: "Smooth, even typing speed.".to_string(),
                    new_keys: vec![],
                    text: "warm winter wind whispers past western pines, while purple petals wake;".to_string(),
                },
                Exercise {
                    id: "8_4".to_string(),
                    title: "Lesson 8 Test".to_string(),
                    exercise_type: ExerciseType::LessonTest,
                    instruction: "Check your progress with W and P.".to_string(),
                    new_keys: vec![],
                    text: "wise people write powerful words upon paper with patience and purpose;".to_string(),
                },
            ],
        },

        // Lesson 9: Keys Q and Y
        Lesson {
            id: "lesson_9".to_string(),
            number: 9,
            title: "Keys Q and Y".to_string(),
            subtitle: "Top left corner and center top reaches".to_string(),
            keys_introduced: vec!['q', 'y'],
            exercises: vec![
                Exercise {
                    id: "9_1".to_string(),
                    title: "Key Intro: Q and Y".to_string(),
                    exercise_type: ExerciseType::KeyIntro,
                    instruction: "Left pinky reaches up to Q; right index reaches up-left to Y.".to_string(),
                    new_keys: vec!['q', 'y'],
                    text: "aqa jyj aqa jyj qqq yyy quit yell quay you quick yard".to_string(),
                },
                Exercise {
                    id: "9_2".to_string(),
                    title: "Q and Y Drills".to_string(),
                    exercise_type: ExerciseType::KeyDrill,
                    instruction: "Remember Q is almost always followed by U!".to_string(),
                    new_keys: vec!['q', 'y'],
                    text: "queen, youth, quote, yellow, quiet, equal, quality, supply, qualify".to_string(),
                },
                Exercise {
                    id: "9_3".to_string(),
                    title: "Sentences with Q and Y".to_string(),
                    exercise_type: ExerciseType::SentenceDrill,
                    instruction: "Flow through complex syllables.".to_string(),
                    new_keys: vec![],
                    text: "the quick young queen quietly enjoys yellow sunlight in the yard;".to_string(),
                },
                Exercise {
                    id: "9_4".to_string(),
                    title: "Lesson 9 Test".to_string(),
                    exercise_type: ExerciseType::LessonTest,
                    instruction: "Test on Q, Y, and vocabulary.".to_string(),
                    new_keys: vec![],
                    text: "quality questions yield quietly unique answers beyond any doubt;".to_string(),
                },
            ],
        },

        // Lesson 10: Keys Z and X
        Lesson {
            id: "lesson_10".to_string(),
            number: 10,
            title: "Keys Z and X".to_string(),
            subtitle: "Bottom row pinky and ring finger reaches".to_string(),
            keys_introduced: vec!['z', 'x'],
            exercises: vec![
                Exercise {
                    id: "10_1".to_string(),
                    title: "Key Intro: Z and X".to_string(),
                    exercise_type: ExerciseType::KeyIntro,
                    instruction: "Left pinky reaches down to Z; left ring reaches down to X.".to_string(),
                    new_keys: vec!['z', 'x'],
                    text: "aza sxs aza sxs zzz xxx zap box fix zip six wax".to_string(),
                },
                Exercise {
                    id: "10_2".to_string(),
                    title: "Z and X Drills".to_string(),
                    exercise_type: ExerciseType::KeyDrill,
                    instruction: "Reach down diagonally without lifting your hand away.".to_string(),
                    new_keys: vec!['z', 'x'],
                    text: "zero, extra, prize, exact, freeze, index, amaze, tax, mix, exit, oxygen".to_string(),
                },
                Exercise {
                    id: "10_3".to_string(),
                    title: "Sentences with All 26 Letters".to_string(),
                    exercise_type: ExerciseType::SentenceDrill,
                    instruction: "You now know the entire alphabet!".to_string(),
                    new_keys: vec![],
                    text: "the quick brown fox jumps over the lazy dog in zero extra seconds;".to_string(),
                },
                Exercise {
                    id: "10_4".to_string(),
                    title: "Alphabet Mastery Test".to_string(),
                    exercise_type: ExerciseType::LessonTest,
                    instruction: "All 26 letters of the English alphabet in action.".to_string(),
                    new_keys: vec![],
                    text: "pack my box with five dozen liquor jugs; crazy typing fixes every zero;".to_string(),
                },
            ],
        },

        // Lesson 11: Capital Letters & Shift
        Lesson {
            id: "lesson_11".to_string(),
            number: 11,
            title: "Capital Letters & Shift".to_string(),
            subtitle: "Coordinating opposite Shift keys with pinky fingers".to_string(),
            keys_introduced: vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'],
            exercises: vec![
                Exercise {
                    id: "11_1".to_string(),
                    title: "Shift Key Technique".to_string(),
                    exercise_type: ExerciseType::KeyIntro,
                    instruction: "Use opposite Shift key: Right Shift for left-hand letters, Left Shift for right-hand letters.".to_string(),
                    new_keys: vec!['A', 'J'],
                    text: "Asdf Jkl; Apple Jack Boston Paris London Tokyo Rome Chicago".to_string(),
                },
                Exercise {
                    id: "11_2".to_string(),
                    title: "Proper Names & Titles".to_string(),
                    exercise_type: ExerciseType::WordDrill,
                    instruction: "Practice fluid Shift coordination on names and places.".to_string(),
                    new_keys: vec![],
                    text: "Alice, Bob, Charlie, David, Emily, Frank, Grace, Henry, Ivy, Jack, Karen, Leo".to_string(),
                },
                Exercise {
                    id: "11_3".to_string(),
                    title: "Full Capitalized Sentences".to_string(),
                    exercise_type: ExerciseType::SentenceDrill,
                    instruction: "Type natural prose with proper capitalization and punctuation.".to_string(),
                    new_keys: vec![],
                    text: "In July, Sarah and David traveled to Mount Rainier in Washington state.".to_string(),
                },
                Exercise {
                    id: "11_4".to_string(),
                    title: "Shift Mastery Test".to_string(),
                    exercise_type: ExerciseType::LessonTest,
                    instruction: "Test on swift capital letter typing.".to_string(),
                    new_keys: vec![],
                    text: "Every Great Journey Begins With A Single Step. Practice Brings Mastery.".to_string(),
                },
            ],
        },

        // Lesson 12: Numbers & Symbols
        Lesson {
            id: "lesson_12".to_string(),
            number: 12,
            title: "Numbers and Punctuation".to_string(),
            subtitle: "Number row (1-0) and essential punctuation (! ? . - ' \")".to_string(),
            keys_introduced: vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '.', '!', '?', '-', '\''],
            exercises: vec![
                Exercise {
                    id: "12_1".to_string(),
                    title: "Number Row Intro".to_string(),
                    exercise_type: ExerciseType::KeyIntro,
                    instruction: "Reach up to the number row with corresponding fingers (1-5 left, 6-0 right).".to_string(),
                    new_keys: vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
                    text: "123 456 789 012 1984 2026 365 24 7 100 500 1000".to_string(),
                },
                Exercise {
                    id: "12_2".to_string(),
                    title: "Numbers & Dates".to_string(),
                    exercise_type: ExerciseType::WordDrill,
                    instruction: "Type numbers mixed with words and symbols.".to_string(),
                    new_keys: vec!['-', '.'],
                    text: "Room 101, Flight 747, Year 2026, 3.14159, 100% effort, 24-hour service.".to_string(),
                },
                Exercise {
                    id: "12_3".to_string(),
                    title: "Full Punctuation Practice".to_string(),
                    exercise_type: ExerciseType::SentenceDrill,
                    instruction: "Combine quotes, question marks, and exclamation points.".to_string(),
                    new_keys: vec!['!', '?', '\'', '"'],
                    text: "Can you believe it? \"Typing is awesome!\" She said, 'I can type 60 WPM!'".to_string(),
                },
                Exercise {
                    id: "12_4".to_string(),
                    title: "Grand Touch Typing Championship Test".to_string(),
                    exercise_type: ExerciseType::LessonTest,
                    instruction: "The ultimate touch typing test covering every key on the keyboard!".to_string(),
                    new_keys: vec![],
                    text: "Congratulations! You have mastered the entire keyboard: all 26 letters, 10 numbers, and full punctuation. Keep practicing every day to reach 100+ WPM!".to_string(),
                },
            ],
        },
    ]
}
