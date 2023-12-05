use std::{error::Error, collections::HashMap};

use itertools::Itertools;
use nom::{bytes::complete::tag, branch::alt, IResult, sequence::terminated};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {

    let mut line = 0;

    let mut sum = 0;

    for game in _input.lines() {
        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;

        let mut isValid: bool = true;

        let parts: Vec<&str> = game.split(';').collect();

        for part in parts {
            let colors: Vec<&str> = part.trim().split(',').collect();

            for color in colors {
                let color_parts: Vec<&str> = color.trim().split_whitespace().collect();
                if color_parts.len() >= 2 {
                    match color_parts[1].trim() {
                        "red" => {
                            red_count += color_parts[0].parse::<i32>().unwrap();
                            if(red_count >12) { isValid = false; }
                        },
                        "green" => {
                            green_count += color_parts[0].parse::<i32>().unwrap();
                            if(green_count >13) { isValid = false; }
                        },
                        "blue" => { 
                            blue_count += color_parts[0].parse::<i32>().unwrap();
                            if(blue_count >14) { isValid = false; }
                        }
                            ,
                        _ => {}
                    }
                }
            }
        }



        line+=1;

        println!(
            "Game {}: red={}, green={}, blue={}",
            line,
            red_count,
            green_count,
            blue_count
        );

        if(isValid == true) {
            sum += line;
        }

        isValid = true;
    }
    return Ok(sum.to_string());
}

pub struct Game {
    gamn: u32,
    colors: Colors
}

enum Colors {
    Red,
    Green,
    Blue
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!("8", process(input)?);
        Ok(())
    }
}
