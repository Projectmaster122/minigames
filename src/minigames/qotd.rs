use serde::Deserialize;
use reqwest::blocking::get;
use std::io::{self, Read};

#[derive(Debug, Deserialize)]
struct Quote {
    q: String,
    a: String,
    c: String,
    h: String,

}

pub fn game() {
    let url = "https://zenquotes.io/api/quotes";
    let response = get(url)
        .expect("failed to make request :3")
        .json::<Vec<Quote>>()
        .expect("failed to parse json :3");


    println!("Pick a number between 0 and 49!");

    let mut choice = String::new();

    io::stdin().read_line(&mut choice).expect(":3");

    let choice_int = choice.trim().parse().unwrap_or(0);

    

    if let Some(quote) = response.get(choice_int)  {
        let quote = response.get(choice_int).expect(":3");

        println!("\"{}\" - {}", quote.q, quote.a)
    } else {
        
        println!("Invalid range! (are you sure you didnt type anything under 0 or anything above 49?");
    }

}
