use rand::seq::IndexedRandom;
use rand::{rng};
use std::fs::File;
use std::io::{BufRead, BufReader, self};

fn load_random_word(filename: &str, min_len: usize, max_len: usize) -> Option<String> {
    let file = File::open(filename).expect("could not open word list");
    let reader = BufReader::new(file);

    let words: Vec<String> = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|word| {
            word.len() >= min_len
                && word.len() <= max_len
                && word.chars().all(|c| c.is_ascii_alphabetic())
        })
        .collect();

    let mut rng = rng();
    words.choose(&mut rng).cloned()
}


fn reveal_guess(secret_word: &String, guess: char, guess_progress: &mut [&'static str; 6]) {
    for (i, c) in secret_word.chars().enumerate().take(6) {
        if c == guess && guess_progress[i] == "_" {
            guess_progress[i] = Box::leak(guess.to_string().into_boxed_str());
        }
    }
}


pub fn game() {
    let secret_word = load_random_word("/wordlist.txt", 6, 6).unwrap();
    let mut lives = 8;

    let mut guess_progress = ["_","_","_","_","_","_"];

    let mut wc = false;
    let mut invalid_letters: Vec<String> = Vec::new();

    println!("Welcome to hangman!\n");
    
    loop {
        if wc == true {
            println!("You win!");
            break;
        } else if lives == 0 {
            println!("You lose! The word was: {}", secret_word);
            break;
        }


        println!("{:?}
        Current lives: {},
        Invalid letters: {:?}\n", guess_progress, lives, invalid_letters);

        let mut guess = String::new();

        println!("Enter a letter to guess:");

        io::stdin().read_line(&mut guess)
            .expect(":3 (line 59)");

        let guess_char = guess.chars().next().map(|c| c.to_ascii_lowercase()).unwrap();

        if secret_word.contains(guess_char) {
            reveal_guess(&secret_word, guess_char, &mut guess_progress);
            if !guess_progress.contains(&"_") {
                wc = true
            }
        } else {
            invalid_letters.push(guess_char.to_string());
            lives = lives - 1;
            println!("Incorrect!");

        }
        

    }
    
}