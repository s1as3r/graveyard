use rand::seq::SliceRandom;
use std::{
    io::{self, Write},
    vec,
};

fn main() {
    println!("{}HANGMAN\n", " ".repeat(32));

    #[rustfmt::skip]
    let mut words =  vec![
        "GUM", "SIN", "FOR", "CRY", "LUG", "BYE", "FLY", "UGLY", "EACH", "FROM",
        "WORK", "TALK", "WITH", "SELF", "PIZZA", "THING", "FEIGN", "FIEND",
        "ELBOW", "FAULT", "DIRTY", "BUDGET", "SPIRIT", "QUAINT", "MAIDEN", "ESCORT",
        "PICKAX", "EXAMPLE", "TENSION", "QUININE", "KIDNEY", "REPLICA", "SLEEPER",
        "TRIANGLE", "KANGAROO", "MAHOGANY", "SERGEANT", "SEQUENCE", "MOUSTACHE",
        "DANGEROUS", "SCIENTIST", "DIFFERENT", "QUIESCENT", "MAGISTRATE", "ERRONEOUSLY",
        "LOUDSPEAKER", "PHYTOTOXIC", "MATRIMONIAL", "PARASYMPATHOMIMETIC", "THIGMOTROPISM",
    ];

    words.shuffle(&mut rand::thread_rng());

    let mut current_word = 0;
    let word_count = words.len();

    let mut keep_playing = true;
    while keep_playing {
        play_game(words[current_word].into());
        current_word += 1;

        if current_word == word_count {
            println!("You did all the words!");
            keep_playing = false;
        } else {
            let mut kp_buf = String::new();
            println!("Want another word? (yes/no) ");
            io::stdin()
                .read_line(&mut kp_buf)
                .expect("error reading stdin");
            keep_playing = kp_buf.chars().next().unwrap().to_ascii_lowercase() == 'y'
        }
    }

    println!("It's been fun! Bye for now.")
}

const PHASES: &'static [(&'static str, fn(&mut Canvas))] = &[
    ("First, we draw a head.", draw_head),
    ("Now we draw a body.", draw_body),
    ("Next we draw an arm.", draw_right_arm),
    ("this time it's the other arm.", draw_left_arm),
    ("Now, let's draw the right leg.", draw_right_leg),
    ("This time we draw the left leg.", draw_left_leg),
    ("Now we put up a hand.", draw_left_hand),
    ("Next the other hand.", draw_right_hand),
    ("Now we draw one foot", draw_left_foot),
    ("Here's the other foot -- you're hung!!", draw_right_foot),
];

struct Canvas {
    buffer: Vec<Vec<char>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let mut canv = Self {
            buffer: vec![vec![char::default(); width]; height],
        };
        canv.clear(' ');
        canv
    }

    fn clear(&mut self, fill: char) {
        for row in &mut self.buffer {
            for i in 1..row.len() {
                row.insert(i, fill);
            }
        }
    }

    fn render(&self) -> String {
        self.buffer
            .iter()
            .map(|x| x.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn put(&mut self, c: char, x: usize, y: usize) {
        self.buffer[y][x] = c
    }

    fn init(&mut self) {
        for i in 0..12usize {
            self.put('X', 0, i)
        }

        for i in 0..7usize {
            self.put('X', i, 0)
        }
        self.put('X', 6, 1)
    }
}

fn draw_head(canvas: &mut Canvas) {
    canvas.put('-', 5, 2);
    canvas.put('-', 6, 2);
    canvas.put('-', 7, 2);
    canvas.put('(', 4, 3);
    canvas.put('.', 5, 3);
    canvas.put('.', 7, 3);
    canvas.put(')', 8, 3);
    canvas.put('-', 5, 4);
    canvas.put('-', 6, 4);
    canvas.put('-', 7, 4);
}

fn draw_body(canvas: &mut Canvas) {
    for i in 5..9usize {
        canvas.put('X', 6, i)
    }
}

fn draw_right_arm(canvas: &mut Canvas) {
    for i in 3..7usize {
        canvas.put('\\', i - 1, i)
    }
}

fn draw_left_arm(canvas: &mut Canvas) {
    canvas.put('/', 10, 3);
    canvas.put('/', 9, 4);
    canvas.put('/', 8, 5);
    canvas.put('/', 7, 6);
}

fn draw_right_leg(canvas: &mut Canvas) {
    canvas.put('/', 5, 9);
    canvas.put('/', 4, 10);
}

fn draw_left_leg(canvas: &mut Canvas) {
    canvas.put('\\', 7, 9);
    canvas.put('\\', 8, 10);
}

fn draw_left_hand(canvas: &mut Canvas) {
    canvas.put('\\', 10, 2);
}

fn draw_right_hand(canvas: &mut Canvas) {
    canvas.put('/', 2, 2);
}

fn draw_left_foot(canvas: &mut Canvas) {
    canvas.put('-', 9, 11);
    canvas.put('-', 10, 11);
}

fn draw_right_foot(canvas: &mut Canvas) {
    canvas.put('-', 2, 11);
    canvas.put('-', 3, 11);
}

fn play_game(guess_target: String) {
    let mut wrong_guesses = 0;
    let mut guess_progress = vec!['-'; guess_target.len()];
    let mut guess_list: Vec<char> = vec![];

    let mut gallows = Canvas::new(12, 12);
    gallows.init();

    let mut guess_count = 0;

    loop {
        println!("Here are the letters you used:");
        println!(
            "{}\n",
            guess_list
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
        println!("{}\n", guess_progress.iter().collect::<String>());

        let guess_letter = loop {
            let mut buf = String::new();

            print!("What is your guess? ");
            let _ = io::stdout().flush();
            io::stdin()
                .read_line(&mut buf)
                .expect("error reading stdin");

            let c = buf.chars().next().unwrap().to_ascii_uppercase();
            if !c.is_alphabetic() {
                println!("Only letters are allowed!");
            } else if guess_list.contains(&c) {
                println!("You guessed that letter before!");
            } else {
                break c;
            }
        };

        guess_list.push(guess_letter);
        guess_count += 1;

        let mut guess_word = String::new();

        if guess_target.contains(guess_letter) {
            let indices: Vec<usize> = guess_target
                .char_indices()
                .filter(|&(_, c)| c == guess_letter)
                .map(|(i, _)| i)
                .collect();

            for i in indices {
                guess_progress[i] = guess_letter;
            }

            if guess_progress.iter().collect::<String>() == guess_target {
                println!("You found the word!");
                break;
            } else {
                println!("\n{}\n", guess_progress.iter().collect::<String>());
                while guess_word == String::new() {
                    println!("What is your guess for the word? ");
                    io::stdin()
                        .read_line(&mut guess_word)
                        .expect("error reading stdin");
                    guess_word = guess_word.to_ascii_uppercase().trim().to_string();
                    if !guess_word.chars().all(|x| x.is_alphabetic()) {
                        guess_word = String::new();
                        println!("Only words are allowed!")
                    }
                }
                if guess_word == guess_target {
                    println!("Right!! It took you {} guesses", guess_count);
                    break;
                }
            }
        } else {
            let (comment, draw_part) = PHASES[wrong_guesses];
            println!("{}", comment);
            draw_part(&mut gallows);
            println!("{}", gallows.render());

            wrong_guesses += 1;
            println!("Sorry, that letter isn't in the word.");

            if wrong_guesses == 10 {
                println!("Sorry, you lose. The word was {}", guess_target);
                break;
            }
        }
    }
}
