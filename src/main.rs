use clap::Parser;
mod minigames;


#[derive(Parser)]
struct Cli {
    game: String,
}

fn main() {
    let args = Cli::parse();
    get_n_call_gamemode(args);
}

fn get_n_call_gamemode(args: Cli) {

    match args.game.as_str() {
        "guess" => minigames::guess::game(),
        "unscramble" => minigames::word_scramble::game(),
        "hangman" => minigames::hangman::game(),
        "trivia" => minigames::trivia::game(),
        _ => {
            println!("Invalid gamemode!\n
            Available Gamemodes:\n
            Guess - Guess the number,\n
            Unscramble - unscramble a word!\n
            Hangman - classic game\n
            trivia - Trivia Guess")
        }
    }
}

