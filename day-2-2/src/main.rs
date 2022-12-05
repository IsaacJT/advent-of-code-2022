use std::env;
use std::fs;

enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    IWin,
    ILose,
    Draw,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let input = get_input(&args[1]);
            let mut total_score = 0;
            for (i, game) in input.iter().enumerate() {
                let score = calculate_score(&game.1, &game.0);
                println!("{}: score {}", i, score);

                total_score += score;
            }

            println!();
            println!("Total score: {}", total_score);
        }
        _ => {
            println!("Usage: {} <input file>", args[0]);
        }
    }
}

fn rps_score(rps: &RockPaperScissors) -> u32 {
    match rps {
        RockPaperScissors::Rock => 1,
        RockPaperScissors::Paper => 2,
        RockPaperScissors::Scissors => 3,
    }
}

fn code_to_rps(code: char) -> RockPaperScissors {
    match code {
        'A' | 'a' => RockPaperScissors::Rock,
        'B' | 'b' => RockPaperScissors::Paper,
        'C' | 'c' => RockPaperScissors::Scissors,
        _ => {
            panic!("Invalid code: {}", code);
        }
    }
}

fn code_to_outcome(code: char) -> Outcome {
    match code {
        'X' | 'x' => Outcome::ILose,
        'Y' | 'y' => Outcome::Draw,
        'Z' | 'z' => Outcome::IWin,
        _ => {
            panic!("Invalid code: {}", code);
        }
    }
}

fn decide_rigged_move(outcome: &Outcome, theirs: &RockPaperScissors) -> RockPaperScissors {
    match outcome {
        Outcome::Draw => match theirs {
            RockPaperScissors::Rock => RockPaperScissors::Rock,
            RockPaperScissors::Paper =>  RockPaperScissors::Paper,
            RockPaperScissors::Scissors => RockPaperScissors::Scissors,
        },
        Outcome::IWin => match theirs {
            RockPaperScissors::Rock => RockPaperScissors::Paper,
            RockPaperScissors::Paper =>  RockPaperScissors::Scissors,
            RockPaperScissors::Scissors => RockPaperScissors::Rock,
        },
        Outcome::ILose => match theirs {
            RockPaperScissors::Rock => RockPaperScissors::Scissors,
            RockPaperScissors::Paper => RockPaperScissors::Rock,
            RockPaperScissors::Scissors =>RockPaperScissors::Paper,
        },
    }
}

fn calculate_score(outcome: &Outcome, theirs: &RockPaperScissors) -> u32 {
    let my_move = decide_rigged_move(outcome, theirs);
    let mut score = rps_score(&my_move);

    score += match outcome {
        Outcome::IWin => 6,
        Outcome::Draw => 3,
        Outcome::ILose => 0,
    };

    score
}

fn get_input(filename: &str) -> Vec<(RockPaperScissors, Outcome)> {
    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut choices: Vec<(RockPaperScissors, Outcome)> = vec![];

    for line in lines.iter() {
        if line.len() < 3 {
            continue;
        }

        choices.push((
            code_to_rps(line.chars().nth(0).expect("Invalid input")),
            code_to_outcome(line.chars().nth(2).expect("Invalid input")),
        ));
    }

    choices
}
