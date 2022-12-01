use std::env;
use std::fs;

fn main() {
    /*
     * This list represents the Calories of the food carried by five Elves:
     *
     * - The first Elf is carrying food with 1000, 2000, and 3000 Calories, a
     *   total of 6000 Calories.
     * - The second Elf is carrying one food item with 4000 Calories.
     * - The third Elf is carrying food with 5000 and 6000 Calories, a total of
     *   11000 Calories.
     * - The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a
     *   total of 24000 Calories.
     * - The fifth Elf is carrying one food item with 10000 Calories.
     *
     * In case the Elves get hungry and need extra snacks, they need to know
     * which Elf to ask: they'd like to know how many Calories are being
     * carried by the Elf carrying the most Calories. In the example above,
     * this is 24000 (carried by the fourth Elf).
     *
     * Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
     */

    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let elves = get_input(&args[1]);
            let (index, cals) = get_most_calories(elves);
            println!("Elf with the most calories: {} with {} cals", index, cals);
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

fn get_most_calories(elves: Vec<Vec<u32>>) -> (usize, u32) {
    let mut totals: Vec<(usize, u32)> = Vec::new();

    for (i, elf) in elves.iter().enumerate() {
        let mut total = 0;

        for cal in elf.iter() {
            total += cal;
        }

        totals.push((i, total));
    }

    totals.sort_by(|a, b| b.1.cmp(&a.1));

    totals[0]
}
