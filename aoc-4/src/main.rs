mod input;
mod range;
use regex::Regex;
use crate::range::RangePair;

fn main() {
    let input = input::get_input();
    let re = Regex::new(r"(?x)
        (?P<a1>\d+)-(?P<a2>\d+),
        (?P<b1>\d+)-(?P<b2>\d+)"
    ).unwrap();

    let mut contained_count = 0;
    let mut overlaps_count = 0;
    for (index, line) in input.lines().enumerate() {
        if !re.is_match(line) {
            println!("Line {}: Invalid syntax! Skipping...", index + 1);
            continue;
        }

        for captures in re.captures_iter(line) {
            if let Ok(pair) = RangePair::from(&captures) {
                if pair.0.fully_overlaps_with(&pair.1) {
                    contained_count += 1;
                }
                if pair.0.overlaps_with(&pair.1) {
                    overlaps_count += 1;
                }
            } else {
                println!("Failed to parse ranges on line {}, skipping...", index + 1);
                continue;
            }
        }
    }
    println!("Total number of ranges contained in another is {contained_count}");
    println!("Total number of ranges overlapping with another is {overlaps_count}");
}
