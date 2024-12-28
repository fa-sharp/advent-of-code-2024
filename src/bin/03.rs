advent_of_code::solution!(3);

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{char, u16},
    sequence::{delimited, separated_pair},
    IResult,
};

const MULT_OPEN_INSTRUCTION: &str = "mul(";
const START_INSTRUCTION: &str = "do()";
const STOP_INSTRUCTION: &str = "don't()";

fn parse_mult_pair(input: &str) -> IResult<&str, (u16, u16)> {
    delimited(
        tag(MULT_OPEN_INSTRUCTION),
        separated_pair(u16, char(','), u16),
        char(')'),
    )(input)
}

fn take_until_mult_open(input: &str) -> IResult<&str, &str> {
    take_until(MULT_OPEN_INSTRUCTION)(input)
}

fn take_until_start(input: &str) -> IResult<&str, &str> {
    take_until(START_INSTRUCTION)(input)
}

fn take_until_stop(input: &str) -> IResult<&str, &str> {
    take_until(STOP_INSTRUCTION)(input)
}

pub fn part_one(mut input: &str) -> Option<u64> {
    let mut mult_pairs: Vec<(u16, u16)> = vec![];

    while !input.is_empty() {
        if let Ok((remainder, _)) = take_until_mult_open(input) {
            if let Ok((new_remainder, mult_pair)) = parse_mult_pair(remainder) {
                mult_pairs.push(mult_pair);
                input = new_remainder;
            } else {
                input = &remainder[MULT_OPEN_INSTRUCTION.len()..];
            }
        } else {
            break;
        }
    }

    Some(
        mult_pairs
            .into_iter()
            .fold(0, |acc, pair| acc + pair.0 as u64 * pair.1 as u64),
    )
}

pub fn part_two(mut input: &str) -> Option<u64> {
    let mut mult_pairs: Vec<(u16, u16)> = vec![];

    let mut enabled = true;

    while !input.is_empty() {
        if enabled {
            if let Ok((mult_remainder, _)) = take_until_mult_open(input) {
                if let Ok((stop_remainder, _)) = take_until_stop(input) {
                    if stop_remainder.len() > mult_remainder.len() {
                        input = &stop_remainder[STOP_INSTRUCTION.len()..];
                        enabled = false;
                        continue;
                    }
                }
                if let Ok((new_remainder, mult_pair)) = parse_mult_pair(mult_remainder) {
                    mult_pairs.push(mult_pair);
                    input = new_remainder;
                } else {
                    input = &mult_remainder[MULT_OPEN_INSTRUCTION.len()..];
                }
            } else {
                break;
            }
        } else {
            if let Ok((remainder, _)) = take_until_start(input) {
                input = &remainder[START_INSTRUCTION.len()..];
                enabled = true;
            } else {
                break;
            }
        }
    }

    Some(
        mult_pairs
            .into_iter()
            .fold(0, |acc, pair| acc + pair.0 as u64 * pair.1 as u64),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
