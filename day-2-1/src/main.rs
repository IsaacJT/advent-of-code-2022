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
        'A' | 'a' | 'X' | 'x' => RockPaperScissors::Rock,
        'B' | 'b' | 'Y' | 'y' => RockPaperScissors::Paper,
        'C' | 'c' | 'Z' | 'z' => RockPaperScissors::Scissors,
        _ => {
            panic!("Invalid code: {}", code);
        }
    }
}

fn decide_outcome(mine: &RockPaperScissors, theirs: &RockPaperScissors) -> Outcome {
    match mine {
        RockPaperScissors::Rock => match theirs {
            RockPaperScissors::Rock => Outcome::Draw,
            RockPaperScissors::Paper => Outcome::ILose,
            RockPaperScissors::Scissors => Outcome::IWin,
        },
        RockPaperScissors::Paper => match theirs {
            RockPaperScissors::Rock => Outcome::IWin,
            RockPaperScissors::Paper => Outcome::Draw,
            RockPaperScissors::Scissors => Outcome::ILose,
        },
        RockPaperScissors::Scissors => match theirs {
            RockPaperScissors::Rock => Outcome::ILose,
            RockPaperScissors::Paper => Outcome::IWin,
            RockPaperScissors::Scissors => Outcome::Draw,
        },
    }
}

fn calculate_score(mine: &RockPaperScissors, theirs: &RockPaperScissors) -> u32 {
    let outcome = decide_outcome(mine, theirs);
    let mut score = rps_score(mine);

    score += match outcome {
        Outcome::IWin => 6,
        Outcome::Draw => 3,
        Outcome::ILose => 0,
    };

    score
}

fn get_input(filename: &str) -> Vec<(RockPaperScissors, RockPaperScissors)> {
    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut choices: Vec<(RockPaperScissors, RockPaperScissors)> = vec![];

    for line in lines.iter() {
        if line.len() < 3 {
            continue;
        }

        choices.push((
            code_to_rps(line.chars().nth(0).expect("Invalid input")),
            code_to_rps(line.chars().nth(2).expect("Invalid input")),
        ));
    }

    choices
}
