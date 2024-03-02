extern crate rand; // example in the book working without ...

use rand::Rng;
use std::{io, cmp::Ordering};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    #[warn(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            let the_match = match self {
                Message::Write(_) => {
                    println!("Message::Write");
                    "Write"
                }
                Message::Quit => {
                    println!("Message::Quit");
                    "Quit"
                }
                Message::Move{ .. } => {
                    println!("Message::Move");
                    "Move"
                }
                Message::ChangeColor(_, _, _) => {
                    println!("Message::ChangeColor");
                    "ChangeColor"
                }
            };
            println!("Matched! Message::{}", the_match);
        }
    }

    let mw = Message::Write(String::from("hello"));
    mw.call();
    let mq: Message = Message::Quit;
    mq.call();
}