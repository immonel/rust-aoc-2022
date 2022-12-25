mod input;
use std::ops::Range;
use std::collections::HashSet;

const LOWER_RANGE: Range<u32> = 0x0061..0x007B;
const UPPER_RANGE: Range<u32> = 0x0041..0x005B;
const LOWER_PRIORITY_OFFSET: u32 = 0x0061 - 1;
const UPPER_PRIORITY_OFFSET: u32 = 0x0041 - 27;

#[derive(Debug, Clone)]
struct InvalidItemError;

fn get_priority(c: char) -> Result<u32, InvalidItemError> {
    let value = c as u32;
    if LOWER_RANGE.contains(&value) {
        Ok(value - LOWER_PRIORITY_OFFSET)
    } else if UPPER_RANGE.contains(&value) {
        Ok(value - UPPER_PRIORITY_OFFSET)
    } else {
        Err(InvalidItemError)
    }
}

fn main() {
    let input = input::get_input();
    let mut total_priority_sum = 0;
    for (index, line) in input.lines().enumerate() {
        // Split into compartments
        let ( first_half, second_half ) = line.split_at(line.len() / 2);
        let first_compartment:  HashSet<char> = HashSet::from_iter(first_half.chars());
        let second_compartment: HashSet<char> = HashSet::from_iter(second_half.chars());

        // Find common item types
        let intersection = first_compartment.intersection(&second_compartment);

        let priority_sum = intersection.clone()
            .fold(0, |sum, char| {
                if let Ok(priority) = get_priority(*char) {
                    sum + priority
                } else {
                    println!("Rucksack {}: Invalid item type '{char}' encountered, ignoring...", index + 1);
                    sum
                }
            });
        println!(
            "Rucksack {}: {} common item(s) with a total of {priority_sum} priority",
            index + 1,
            intersection.count()
        );
        total_priority_sum += priority_sum;
    }
    println!("Finished checking rucksacks, total accumulated priority is {total_priority_sum}");
}
