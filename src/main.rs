pub(crate) use std::fmt::{self, Display};

use rand::Rng;

enum Sign {
    Rock,
    Paper,
    Scissors,
}

impl Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Sign::Rock => write!(f, "Rock"),
            Sign::Paper => write!(f, "Paper"),
            Sign::Scissors => write!(f, "Scissors"),
        }
    }
}

enum ResultType {
    Win,
    Loss,
    Draw,
}

struct Result(ResultType, Sign);

fn shoot(sign: Sign) -> Result {
    let bot_sign = match rand::thread_rng().gen_range(1..=3) {
        1 => Sign::Rock,
        2 => Sign::Paper,
        _ => Sign::Scissors,
    };

    let result_type = match sign {
        Sign::Rock => match bot_sign {
            Sign::Rock => ResultType::Draw,
            Sign::Paper => ResultType::Loss,
            Sign::Scissors => ResultType::Win,
        },
        Sign::Paper => match bot_sign {
            Sign::Rock => ResultType::Win,
            Sign::Paper => ResultType::Draw,
            Sign::Scissors => ResultType::Loss,
        },
        Sign::Scissors => match bot_sign {
            Sign::Rock => ResultType::Loss,
            Sign::Paper => ResultType::Win,
            Sign::Scissors => ResultType::Draw,
        },
    };

    Result(result_type, bot_sign)
}

fn main() {
    loop {
        println!("Rock(1), Paper(2), Scissors(3), Shoot:");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed");

        match input.trim() {
            "quit" | "q" | "exit" => break,
            _ => {}
        }

        let user_sign = match input.trim().parse() {
            Ok(1) => Sign::Rock,
            Ok(2) => Sign::Paper,
            Ok(3) => Sign::Scissors,
            Ok(_) | Err(_) => {
                println!("Enter a correct Value.");
                continue;
            },
        };

        match shoot(user_sign) {
            Result(ResultType::Win, x) => println!("You won, I choose {}", x),
            Result(ResultType::Loss, x) => println!("I won , I choose {}", x),
            Result(ResultType::Draw, x) => println!("It's a draw, we both choose {}", x),
        }
    }
}
