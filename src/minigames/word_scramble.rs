use rand::seq::IndexedRandom;
use rand::{seq::SliceRandom, rng};
use std::fs::File;
use std::io::{BufRead, BufReader, self, Write};

fn load_words(filename: &str, min_len: usize, max_len: usize) -> Vec<String> {
    let file = File::open(filename).expect("could not open word list");
    let reader = BufReader::new(file);
    reader
        .lines()
        .filter_map(Result::ok)
        .filter(|word| {
            word.len() >= min_len
                && word.len() <= max_len
                && word.chars().all(|c| c.is_ascii_alphabetic())
        })
        .collect()
}

fn scramble_word(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    let mut rng = rng();
    while {
        chars.shuffle(&mut rng);
        chars.iter().collect::<String>() == word
    } {}
    chars.into_iter().collect()
}

pub fn game() {
    let words = load_words("/wordlist.txt", 4, 10);
    let mut rng = rng();
    
    let word = words.choose(&mut rng).expect("you did some shit and now there are no words :3");
    let scrambled = scramble_word(&word);

    println!("Unscramble the following word: {}", scrambled);

    print!("Yer guess:");
    io::stdout().flush().unwrap();

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).unwrap();
    let guess = guess.trim().to_lowercase();

    if guess == *word {
        println!("correct :3");
    } else {
        println!("nop");
    }
}