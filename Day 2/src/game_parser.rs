use std::str::FromStr;
use crate::game::Game;

#[derive(Debug)]
pub enum GameParseError{
    InvalidFormat(String),
    IDParseError(String),
    ColorParseError(String),
    CountParseError(String),
}

impl Game {
    fn parse_set(s: &str) -> Result<[u32; 3], GameParseError> {
        // Input str example: 'blue 1, red 3, green 1

        let mut set: [u32; 3] = [0,0,0];

        let parts: Vec<&str> = s.split(", ").collect();

        for part in parts {
            let color_split: Vec<&str> = part.split(" ").collect();
            
            if color_split.len() != 2 {
                return Err(GameParseError::InvalidFormat(concat_string!("At '", part ,"'. Ex: 'Game 1: 1 red, 2 green, 3 blue; 1 green, 1 blue".to_string())));
            }

            let count: u32 = color_split[0].parse::<u32>().map_err(|_| GameParseError::CountParseError(concat_string!("Failed to parse cube count: '", color_split[0], "'; must be a non-negative integer")))?;


            match color_split[1] {
                "red" => set[0] += count,
                "green" => set[1] += count,
                "blue" => set[2] += count,
                other => return Err(GameParseError::ColorParseError(concat_string!("Failed to parse color: '", other, "'; must be red, green or blue"))),
            }
        }

        Ok(set)

    }
}

impl FromStr for Game {

    type Err = GameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        let parts: Vec<&str> = s.split(": ").collect();
        if parts.len() != 2 {
            return Err(GameParseError::InvalidFormat(concat_string!("At '", s , "'Ex: 'Game 1: 1 red, 2 green, 3 blue; 1 green, 1 blue".to_string())));
        }

        let id: u8 = parts[0].trim_start_matches("Game ").parse::<u8>().map_err(|_| GameParseError::IDParseError(concat_string!("Failed to parse ID number from '", parts[0], "'")))?;
        let game_sets: Vec<[u32; 3]> = parts[1].split("; ").map(|set: &str| Game::parse_set(set)).collect::<Result<Vec<_>, _>>()?;

        Ok( Game::new(id, game_sets) )
    }
}