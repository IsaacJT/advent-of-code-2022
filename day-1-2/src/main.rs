use std::env;
use std::fs;

fn main() {
    /*
     * Find the top three Elves carrying the most Calories. How many Calories
     * are those Elves carrying in total?
     */

    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let elves = get_input(&args[1]);
            let highest = get_total_calories(elves);
            let mut total = 0;
            for (i, (index, cals)) in highest[..3].iter().enumerate() {
                println!("{}: Elf {} with {} cals", i, index, cals);
                total += cals;
            }

            println!("Total calories: {}", total);
        }
        _ => {
            panic!("Usage: {} <input file>", args[0]);
        }
    }
}

fn get_input(filename: &str) -> Vec<Vec<u32>> {
    let mut elves: Vec<Vec<u32>> = vec![vec![]];
    let mut index = 0;
    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    for line in lines.iter() {
        match line.len() {
            0 => {
                index += 1;
                elves.push(vec![]);
            }
            _ => {
                let val = line.parse::<u32>().expect("Could not parse string");
                elves[index].push(val);
            }
        }
    }

    elves
}

fn get_total_calories(elves: Vec<Vec<u32>>) -> Vec<(usize, u32)> {
    let mut totals: Vec<(usize, u32)> = Vec::new();

    for (i, elf) in elves.iter().enumerate() {
        let mut total = 0;

        for cal in elf.iter() {
            total += cal;
        }

        totals.push((i, total));
    }

    totals.sort_by(|a, b| b.1.cmp(&a.1));

    totals
}
