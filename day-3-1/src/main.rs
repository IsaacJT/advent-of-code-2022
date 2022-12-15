use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let lines = get_input(&args[1]);
            let mut total_priority: u32 = 0;
            let mut common_items: Vec<Vec<char>> = vec![];
            for line in lines.clone() {
                let first_compartment: Vec<char> = line.0.chars().collect();
                let second_compartment: Vec<char> = line.1.chars().collect();
                common_items.push(vec_union(first_compartment, second_compartment));
            }
            for items in common_items.iter() {
                let mut rucksack_priority: u32 = 0;

                for item in items {
                    rucksack_priority += get_item_priority(*item);
                }

                total_priority += rucksack_priority;
            }

            println!("Total priority: {}", total_priority);
        }
        _ => {
            println!("Usage: {} <input file>", args[0]);
        }
    }
}

fn vec_union(mut a: Vec<char>, mut b: Vec<char>) -> Vec<char> {
    let mut common: Vec<char> = vec![];
    a.sort();
    b.sort();

    /* Get common items (chars), but only count each unique char once */
    for item in a.iter() {
        match common.binary_search(item) {
            Ok(_) => continue,
            Err(_) => match b.binary_search(item) {
                Ok(_) => common.push(*item),
                Err(_) => continue,
            },
        }
    }

    common
}

fn get_item_priority(item: char) -> u32 {
    /*
     * Lowercase item types a through z have priorities 1 through 26.
     * Uppercase item types A through Z have priorities 27 through 52.
     */

    assert!(item.is_ascii());

    let priority = match item {
        'a'..='z' => (item as u32) + 1 - ('a' as u32),
        'A'..='Z' => (item as u32) + 27 - ('A' as u32),
        _ => {
            panic!("'{}' is not alphabetic", item);
        }
    };

    priority
}

fn get_input(filename: &str) -> Vec<(String, String)> {
    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut rucksacks: Vec<(String, String)> = vec![];
    for line in lines {
        if line.len() == 0 {
            continue;
        }

        if (line.len() & 1) != 0 {
            panic!("Line length must be an even number!");
        }

        let half_len = line.len() / 2;
        let new_rucksack = (line[..half_len].to_owned(), line[half_len..].to_owned());

        assert_eq!(new_rucksack.0.len(), new_rucksack.1.len());

        rucksacks.push(new_rucksack);
    }
    rucksacks
}
