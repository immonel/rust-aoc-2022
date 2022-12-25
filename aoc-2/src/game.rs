use crate::Outcome::*;
use crate::Shape::*;

#[derive(Copy, Clone, Debug)]
pub enum Shape {
    Rock     = 1,
    Paper    = 2,
    Scissors = 3
}

#[derive(Copy, Clone)]
pub enum Outcome {
    Loss = 0,
    Draw = 3,
    Win  = 6
}

pub struct Turn {
  pub opponent: Shape,
  pub outcome: Outcome,
}

#[derive(Debug, Clone)]
pub struct InvalidTurnError;

impl Turn {
  pub fn new(opponent: char, outcome: char) -> Result<Turn, InvalidTurnError> {
    Ok(Turn {
      outcome: match outcome {
        'X' => Loss,
        'Y' => Draw,
        'Z' => Win,
        _   => return Err(InvalidTurnError)
      },
      opponent: match opponent {
        'A' => Rock,
        'B' => Paper,
        'C' => Scissors,
        _   => return Err(InvalidTurnError)
      }
    })
  }

  pub fn player_shape(&self) -> Shape {
    match self.opponent {
      Rock => match self.outcome {
      Loss => Scissors,
      Draw => Rock,
        Win  => Paper
      },
      Paper => match self.outcome {
        Loss => Rock,
        Draw => Paper,
        Win  => Scissors
      },
      Scissors => match self.outcome {
        Loss => Paper,
        Draw => Scissors,
        Win  => Rock
      }
    }
  }

  pub fn shape_score(&self) -> u32 {
      self.player_shape() as u32
  }

  pub fn outcome(&self) -> Outcome {
      self.outcome
  }

  pub fn total_score(&self) -> u32 {
    self.shape_score() as u32 +
    self.outcome()     as u32
  }
}