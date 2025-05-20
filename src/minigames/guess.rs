use rand::Rng;
use std::io;



pub fn game() {
    println!("Please input maximum range for guessing game: (2-1000)");

    let mut range: String = Default::default();
    
    io::stdin().read_line(&mut range)
        .expect("rust shit happened :3");
    
    let range_int: u32 = range.trim().parse()
    .expect("funky shit happened :3");

    let secret_number = rand::rng().random_range(1..range_int);

    loop {
        println!("Guess a number between 1 and {}", range);

        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess)
            .expect(":3");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("you make me angy, i told you type number but you no type number!!! >:3");
                continue;
            }
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("too little!\n"),
            std::cmp::Ordering::Greater => println!("too much!"),
            std::cmp::Ordering::Equal => {
                println!("ye hast won");
                break;
            }
        }   
    }
}