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
  pub player: Shape,
}

#[derive(Debug, Clone)]
pub struct InvalidTurnError;

impl Turn {
  pub fn new(opponent: char, player: char) -> Result<Turn, InvalidTurnError> {
    Ok(Turn {
      player: match player {
        'X' => Rock,
        'Y' => Paper,
        'Z' => Scissors,
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

  pub fn shape_score(&self) -> u32 {
      self.player as u32
  }

  pub fn outcome(&self) -> Outcome {
      match self.player {
          Rock => match self.opponent {
              Rock     => Draw,
              Paper    => Loss,
              Scissors => Win
          },
          Paper => match self.opponent {
              Rock     => Win,
              Paper    => Draw,
              Scissors => Loss
          },
          Scissors => match self.opponent {
              Rock     => Loss,
              Paper    => Win,
              Scissors => Draw
          }   
      }
  }

  pub fn total_score(&self) -> u32 {
    self.shape_score() as u32 +
    self.outcome()     as u32
  }
}