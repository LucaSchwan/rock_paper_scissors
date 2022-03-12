use std::fmt::{self, Display};

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
    let bot_sign = match rand::thread_rng().gen_range(0..3) {
        0 => Sign::Rock,
        1 => Sign::Paper,
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

    return Result(result_type, bot_sign);
}

fn get_input(input: &mut i32) {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    if let Ok(x) = buffer.trim().parse::<i32>() {
        *input = x
    } else {
        *input = 0
    }
}

fn main() {
    println!("Rock(1), Paper(2), Scissors(3) Shoot:");
    let mut input: i32 = 0;
    get_input(&mut input);

    while !(1..=3).contains::<i32>(&input) {
        println!("Please enter a valid input");
        get_input(&mut input);
    }

    let user_sign = match input {
        1 => Sign::Rock,
        2 => Sign::Paper,
        _ => Sign::Scissors,
    };

    match shoot(user_sign) {
        Result(ResultType::Win, x) => println!("You won, I choose {}", x),
        Result(ResultType::Loss, x) => println!("I won , I choose {}", x),
        Result(ResultType::Draw, _) => println!("It's a draw"),
    }
}
