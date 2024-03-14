extern crate rand; // example in the book working without ...
extern crate clap;

use rand::Rng;
use std::{io, cmp::Ordering};
use clap::Parser;


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


/// Parse the command line option
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Execute Guess game
    #[arg(short, long)]
    guess: bool,

    /// Execute simple match
    #[arg(short, long, default_value_t = false)]
    matcher: bool,
}

fn parse_args() {
    let args = Cli::parse();

    println!("args: {:?}", args);

    if args.guess {
        println!("Guess activated");
        guesser();
    }

    if args.matcher {
        println!("Match activated");
        matcher();
    }
}


fn main() {
    parse_args();
}

fn guesser() {
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
}

fn matcher() {
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
                Message::Write(string) => {
                    println!("Message::Write: \"{}\"", string);
                    "Write"
                }
                Message::Quit => {
                    println!("Message::Quit");
                    "Quit"
                }
                Message::Move{ x, y } => {
                    println!("Message::Move({}, {})", x, y);
                    "Move"
                }
                Message::ChangeColor(r, g, b) => {
                    println!("Message::ChangeColor: ({},{},{})", r, g, b);
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
    let mm = Message::Move { x: (4), y: (5) };
    mm.call();
    let mc = Message::ChangeColor(1, 2, 3);
    mc.call();
}