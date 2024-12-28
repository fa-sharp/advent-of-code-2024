use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_nums, mut right_nums) = parse_input(input);

    left_nums.sort();
    right_nums.sort();

    let total_distance = left_nums
        .iter()
        .zip(right_nums)
        .fold(0, |acc, (left, right)| acc + left.abs_diff(right));

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_nums, right_nums) = parse_input(input);

    let mut frequencies: HashMap<u32, u32> = HashMap::new();
    for &num in right_nums.iter() {
        *frequencies.entry(num).or_insert(0) += 1;
    }

    let similarity_score = left_nums
        .iter()
        .fold(0, |acc, num| acc + num * frequencies.get(num).unwrap_or(&0));

    Some(similarity_score)
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|val| val.parse::<u32>().expect("Should be positive integer"));
            (
                nums.next().expect("Should have left number"),
                nums.next().expect("Should have right number"),
            )
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
