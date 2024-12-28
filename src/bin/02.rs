advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let num_safe_reports = input
        .lines()
        .filter(|line| {
            let levels: Vec<i8> = line
                .split_whitespace()
                .map(|n| n.parse().expect("Level should be integer"))
                .collect();

            is_safe_sequence(&levels)
        })
        .count() as u32;

    Some(num_safe_reports)
}

pub fn part_two(input: &str) -> Option<u32> {
    let num_safe_reports = input
        .lines()
        .filter(|line| {
            let levels: Vec<i8> = line
                .split_whitespace()
                .map(|n| n.parse().expect("Level should be integer"))
                .collect();

            if is_safe_sequence(&levels) {
                return true;
            }

            for i in 0..levels.len() {
                let mut levels_modified = levels.clone();
                levels_modified.remove(i);
                if is_safe_sequence(&levels_modified) {
                    return true;
                }
            }

            false
        })
        .count() as u32;

    Some(num_safe_reports)
}

fn is_safe_sequence(levels: &[i8]) -> bool {
    levels.windows(3).all(|w| {
        let diff1 = w[1] - w[0];
        let diff2 = w[2] - w[1];
        diff1.signum() == diff2.signum()
            && diff1.abs() >= 1
            && diff1.abs() <= 3
            && diff2.abs() >= 1
            && diff2.abs() <= 3
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
