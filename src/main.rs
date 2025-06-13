use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;
use rand::seq::SliceRandom;
use rand::thread_rng;

// hangman figure stages
const HANGMAN_PICS: [&str; 7] = [
    r#"
 +---+
 |   |
     |
     |
     |
     |
========="#,
    r#"
 +---+
 |   |
 O   |
     |
     |
     |
========="#,
    r#"
 +---+
 |   |
 O   |
 |   |
     |
     |
========="#,
    r#"
 +---+
 |   |
 O   |
/|   |
     |
     |
========="#,
    r#"
 +---+
 |   |
 O   |
/|\  |
     |
     |
========="#,
    r#"
 +---+
 |   |
 O   |
/|\  |
/    |
     |
========="#,
    r#"
 +---+
 |   |
 O   |
/|\  |
/ \  |
     |
========="#,
];

// loads a list of words from the specified text file and returns them as a Vec<String>
fn load_words(filename: &str) -> Vec<String> {
    let path = Path::new(filename);
    let file = fs::File::open(&path).expect("Could not open word list file.");
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect()
}

// main program loop that displays main menu and handles menu selection
fn main() {
    loop {
        println!("=== Hangman ===");
        println!("1. Play Hangman");
        println!("2. Quit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => play_hangman(),
            "2" => {
                println!("Goodbye!");
                break;
            },
            _ => println!("Invalid choice. Please enter 1 or 2."),
        }
    }
}

// displays difficulty selection menu, loads word list, starts game
fn play_hangman() {
    println!("Choose difficulty:");
    println!("1. Easy");
    println!("2. Medium");
    println!("3. Hard");

    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let mut difficulty = String::new();
    io::stdin().read_line(&mut difficulty).expect("Failed to read input");
    let difficulty = difficulty.trim();

    let word_list = match difficulty {
        "1" => load_words("words/easy.txt"),
        "2" => load_words("words/medium.txt"),
        "3" => load_words("words/hard.txt"),
        _ => {
            println!("Invalid difficulty choice.");
            return;
        }
    };

    let mut rng = thread_rng();
    let word = word_list.choose(&mut rng).unwrap();

    play_game(word);
}

// runs the main hangman game loop. tracks guesses, displays word progress and hangman figure, checks win/loss
fn play_game(word: &str) {
    let mut attempts_remaining = 6;
    let mut guessed_letters: Vec<char> = Vec::new();

    println!("The word has {} letters.", word.len());

    loop {
        // display current word progress
        let mut display_word = String::new();
        let mut all_guessed = true;

        for c in word.chars() {
            if guessed_letters.contains(&c) {
                display_word.push(c);
            } else {
                display_word.push('_');
                display_word.push(' ');
                all_guessed = false;
            }
        }

        println!("Word: {}", display_word.trim());
        println!("Attempts remaining: {}", attempts_remaining);

        // display hangman figure
        let stage = 6 - attempts_remaining;
        println!("{}", HANGMAN_PICS[stage]);

        // check for win
        if all_guessed {
            println!("🎉 You won! The word was '{}'.", word);
            break;
        }

        // check for lose
        if attempts_remaining == 0 {
            println!("💀 You lost! The word was '{}'.", word);
            break;
        }

        // prompt for guess
        print!("Enter a letter: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");
        let guess = guess.trim().chars().next();

        match guess {
            Some(c) if c.is_ascii_alphabetic() => {
                let c = c.to_ascii_lowercase();

                if guessed_letters.contains(&c) {
                    println!("You already guessed '{}'.", c);
                } else {
                    guessed_letters.push(c);

                    if !word.contains(c) {
                        println!("Incorrect guess!");
                        attempts_remaining -= 1;
                    }
                }
            }
            _ => {
                println!("Please enter a valid letter.");
            }
        }

        println!(); // blank line between turns
    }
}