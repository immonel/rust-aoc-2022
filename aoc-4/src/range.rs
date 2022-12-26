use std::num::ParseIntError;

pub struct RangePair (pub Range, pub Range);

impl RangePair {
    pub fn from(captures: &regex::Captures) -> Result<RangePair, ParseIntError> {
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

pub struct Range {
    start: i32,
    end: i32
}

impl Range {
    pub fn is_contained_by(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    pub fn fully_overlaps_with(&self, other: &Self) -> bool {
        self.is_contained_by(other) || other.is_contained_by(&self)
    }

    pub fn overlaps_with(&self, other: &Self) -> bool {
        (self.start  >= other.start && self.start  <= other.end) || 
        (other.start >= self.start  && other.start <= self.end)
    }
}