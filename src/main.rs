extern crate rand; // example in the book working without ...

use rand::Rng;
use std::{io, cmp::Ordering};
use std::env;


pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}


fn help() {
    println!("usage:
pass a single argument or none whatsoever")
}

fn parse_args() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments passed
        1 => {
            println!("Try passing some arguments!");
        },
        // one argument passed
        2 => {
            match args[1].as_str() {
                "-i" => println!("Short option!"),
                "--interactive" => println!("Long option!"),
                _ => println!("Unrecognized option.")
            }
        },
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}


fn main() {
    parse_args();

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