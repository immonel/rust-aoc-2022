use std::fs::File;
use std::io::{self, Read};
use std::process;
use itertools::sorted;

const FILE_PATH: &str = "input.txt";

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Elf {
    calories: u32
}

impl Elf {
    fn new(calories: u32) -> Self {
        Self { calories }
    }    
}

fn read_file_contents(file_path: &str) -> Result<String, io::Error> {
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Returns the sum of calories held by top n elves wrapped in Option
/// or None if number of elves is less than n
fn top_n_calorie_count(elves: &Vec<Elf>, n: usize) -> Option<u32> {
    if elves.len() < n {
        return None
    }
    let sorted_elves: Vec<&Elf> = sorted(elves)
        .rev()
        .collect();

    Some (
        sorted_elves[..n]
            .iter()
            .map(|elf| elf.calories)
            .sum()
    )
}

fn main() {
    let contents = read_file_contents(FILE_PATH);

    let contents = match contents {
        Err(error) => {
            println!("Failed to read {FILE_PATH}:\n{error}");
            process::exit(1);
        },
        Ok(str) => str
    };

    let mut elves: Vec<Elf> = vec![];
    let mut calorie_count: u32 = 0;
    for line in contents.lines() {
        if line.trim().len() == 0 { // line is empty
            elves.push(Elf::new(calorie_count));
            calorie_count = 0;
        }

        // Ignore non-numbers
        if let Ok(number) = line.parse::<u32>() {
            calorie_count += number;
        }
    }
    elves.push(Elf::new(calorie_count));
    
    // Part 1 -- top calories
    println!("Part 1: top calories");
    if let Some(count) = top_n_calorie_count(&elves, 1) {
        println!("-- Max calorie count is {count}");
    } else {
        println!("-- There were no elves!");
    }

    // Part 2 -- top three
    println!("Part 2: top three");
    if let Some(count) = top_n_calorie_count(&elves, 3) {
        println!("-- Top three elves hold total of {count} calories");
    } else {
        println!("-- There were less than three elves!");
    }
}
