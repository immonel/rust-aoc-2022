mod input;
mod game;
use game::Shape;
use game::Turn;
use game::Outcome;

#[derive(Debug, Clone)]
struct ParseError;

fn parse_line(line: &str) -> Result<(char, char), ParseError> {
    let substrings: Vec<&str> = line.split(' ').collect();
    if substrings.len() != 2 {
        return Err(ParseError)
    }
    let turn: Vec<char> = substrings[..]
        .iter()
        .map(|str| str
            .chars()
            .nth(0)
            .unwrap_or('?'))
        .collect();
        
    Ok((turn[0], turn[1]))
}

fn main() {
    let input = input::get_input();
    let mut score = 0;

    for (index, line) in input.lines().enumerate() {
        let ( opponent_shape, player_shape ) = match parse_line(line) {
            Ok(shapes) => shapes,
            Err(_) => {
                println!("Turn {}: Invalid number of turn parameters, skipping turn!", index + 1);
                continue;
            }
        };

        let turn = match Turn::new(opponent_shape, player_shape) {
            Ok(turn) => turn,
            Err(_) => {
                println!("Turn {}: Invalid turn parameters, skipping turn!", index + 1);
                continue;
            }
        };

        println!("Turn {}: {:?} vs. {:?}\t--> {} points, {score} total",
            index + 1,
            turn.player,
            turn.opponent,
            turn.total_score()
        );
        score += turn.total_score();
    }
    println!("Game finished! Total score is {score}.");
}
