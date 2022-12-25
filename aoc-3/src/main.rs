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

fn calculate_priority_sum(set: &HashSet<char>) -> u32 {
    set.iter().fold(0, |sum, char| {
        if let Ok(priority) = get_priority(*char) {
            sum + priority
        } else {
            println!("Invalid item type '{char}' encountered, ignoring...");
            sum
        }
    })
}

fn main() {
    let input = input::get_input();
    let mut total_priority_sum = 0;
    let mut badge_priority_sum = 0;
    let mut group_common_items: HashSet<char> = HashSet::new();

    for (index, line) in input.lines().enumerate() {
        // Split into compartments
        let ( first_half, second_half ) = line.split_at(line.len() / 2);
        let first_compartment:  HashSet<char> = HashSet::from_iter(first_half.chars());
        let second_compartment: HashSet<char> = HashSet::from_iter(second_half.chars());

        // Find common item types (intersection)
        let intersection = &first_compartment & &second_compartment;

        let priority_sum = calculate_priority_sum(&intersection);
        println!(
            "Rucksack {}: {} common item(s) with a total of {priority_sum} priority",
            index + 1,
            intersection.iter().count()
        );
        total_priority_sum += priority_sum;

        // Part 2 -- find badges from groups
        let item_set: HashSet<char> = HashSet::from_iter(line.chars());
        if index % 3 == 0 { // Start of group
            group_common_items = item_set.clone();
        } else {
            group_common_items = &group_common_items & &item_set;
        }
        if index % 3 == 2 { // End of group
            let badge_priority = calculate_priority_sum(&group_common_items);
            println!("Group {}: Badge priority is {badge_priority}", (index / 3) + 1);
            badge_priority_sum += badge_priority;
        }
    }
    println!("Finished checking rucksacks, total accumulated priority is {total_priority_sum}");
    println!("Total badge priority sum is {badge_priority_sum}");
}
