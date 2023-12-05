use std::{error::Error, collections::HashMap};

use itertools::Itertools;
use nom::{bytes::complete::tag, branch::alt, IResult, sequence::terminated};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {

    for line in _input.lines() {
        
        line.

    }

    return Ok("".to_string());
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input =  "467..114..
                            ...*......
                            ..35..633.
                            ......#...
                            617*......
                            .....+.58.
                            ..592.....
                            ......755.
                            ...$.*....
                            .664.598..";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
