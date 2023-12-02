use std::error::Error;

use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut total = 0;

    for line in _input.lines() {
        let first = line
            .chars()
            .into_iter()
            .find_or_first(|char| char.is_numeric());
        let last = line
            .chars()
            .into_iter()
            .rev() //flip it around
            .find_or_last(|char| char.is_numeric());

        match (first, last) {
            (None, None) => todo!(),
            (None, Some(_)) => todo!(),
            (Some(_), None) => todo!(),
            (Some(_), Some(_)) => {
                let validationCode = first.unwrap().to_string() + &last.unwrap().to_string();

                total += validationCode.parse::<i32>().unwrap();
            }
        }
    }

    return Ok(total.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        assert_eq!("142", process(input)?);
        Ok(())
    }
}
