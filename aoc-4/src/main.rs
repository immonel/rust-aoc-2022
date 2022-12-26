mod input;
use regex::Regex;
use std::num::ParseIntError;

struct RangePair (Range, Range);

impl RangePair {
    fn from(captures: &regex::Captures) -> Result<RangePair, ParseIntError> {
        Ok(RangePair(
            Range {
                start:  captures[1].parse()?,
                end:    captures[2].parse()?
            },
            Range {
                start:  captures[3].parse()?,
                end:    captures[4].parse()?
            }
        ))
    }
}

struct Range {
    start: i32,
    end: i32
}

impl Range {
    fn is_contained_by(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    fn fully_overlaps_with(&self, other: &Self) -> bool {
        self.is_contained_by(other) || other.is_contained_by(&self)
    }

    fn overlaps_with(&self, other: &Self) -> bool {
        (self.start  >= other.start && self.start  <= other.end) || 
        (other.start >= self.start  && other.start <= self.end)
    }
}

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
