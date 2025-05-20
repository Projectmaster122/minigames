use serde::Deserialize;
use reqwest::blocking::get;
use std::io;

use crate::minigames::guess;


#[derive(Debug, Deserialize)]
struct TriviaResponse {
    response_code: u8,
    results: Vec<TriviaQuestion>,
}

#[derive(Debug, Deserialize)]
struct TriviaQuestion {
    category: String,
    #[serde(rename = "type")]
    question_type: String,
    difficulty: String,
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}

pub fn game() {
    let url = "https://opentdb.com/api.php?amount=1&type=multiple";
    let response = get(url).unwrap().json::<TriviaResponse>().unwrap();

    let question = &response.results[0];


    let decoded_question = htmlescape::decode_html(&question.question).expect(":3");
    let decoded_correct = htmlescape::decode_html(&question.correct_answer).expect(":3");
    // println!("{}\n{}\n{}\n{:?}\n{}\n{}",question.category,question.correct_answer,question.difficulty,question.incorrect_answers,question.question,question.question_type);

    println!("Question: {}", decoded_question);
    let mut answers: Vec<String> = question
    .incorrect_answers
    .iter()
    .map(|ans| htmlescape::decode_html(ans).unwrap_or_else(|_| ans.clone()))
    .collect();

    answers.insert(0, decoded_correct.clone());
    answers.sort();

    println!("{:?}", answers);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("funky shit :3");

    let  guess_int: usize = guess.trim().parse().unwrap_or(0);

    if guess_int > 0 && guess_int <= answers.len() && answers[guess_int-1] == question.correct_answer {
        println!("correct!")
    } else {
        println!("no you idiot")
    }

}