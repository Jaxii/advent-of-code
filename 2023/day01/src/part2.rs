use aho_corasick::AhoCorasick;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let output =
        input.lines().map(process_line).sum::<u32>();

    Ok(output.to_string())
}

const PATTERNS: [&str; 19] = [
    "one", "two", "three", "four", "five", "six", "seven",
    "eight", "nine", "0", "1", "2", "3", "4", "5", "6",
    "7", "8", "9",
];

#[tracing::instrument]
fn process_line(line: &str) -> u32 {
    let ac = AhoCorasick::new(PATTERNS).unwrap();

    let mut it = ac.find_overlapping_iter(line);
    let first = from_matchables(
        PATTERNS[it
            .next()
            .expect("should be a number")
            .pattern()],
    );

    match it
        .last()
        .map(|mat| from_matchables(PATTERNS[mat.pattern()]))
    {
        Some(num) => first * 10 + num,
        None => first * 10 + first,
    }
}

#[tracing::instrument]
fn from_matchables(input: &str) -> u32 {
    match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        _ => panic!("unexpected number!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
