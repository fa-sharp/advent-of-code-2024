advent_of_code::solution!(4);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }
    grid
}

fn num_xmas(grid: &Vec<Vec<char>>) -> u32 {
    let mut total: u32 = 0;

    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let target = ['X', 'M', 'A', 'S'];
    let directions = [
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // bottom
        (-1, 0),  // top
        (1, 1),   // bottom-right
        (1, -1),  // bottom-left
        (-1, 1),  // top-right
        (-1, -1), // top-left
    ];

    for &(row_offset, col_offset) in directions.iter() {
        let (row_start, row_end) = if row_offset > 0 {
            (0, num_rows - (target.len() - 1))
        } else if row_offset < 0 {
            (target.len() - 1, num_rows)
        } else {
            (0, num_rows)
        };

        let (col_start, col_end) = if col_offset > 0 {
            (0, num_cols - (target.len() - 1))
        } else if col_offset < 0 {
            (target.len() - 1, num_cols)
        } else {
            (0, num_cols)
        };

        for row in row_start..row_end {
            for col in col_start..col_end {
                let mut matches = true;
                for i in 0..4 {
                    let r = (row as i32 + i as i32 * row_offset) as usize;
                    let c = (col as i32 + i as i32 * col_offset) as usize;
                    if grid[r][c] != target[i] {
                        matches = false;
                        break;
                    }
                }
                if matches {
                    total += 1;
                }
            }
        }
    }

    total
}

fn num_cross_mas(grid: &Vec<Vec<char>>) -> u32 {
    let mut total: u32 = 0;

    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let target = ["SAM".to_owned(), "MAS".to_owned()];

    for row in 1..num_rows - 1 {
        for col in 1..num_cols - 1 {
            if grid[row][col] == 'A' {
                let diag1: String = [grid[row - 1][col - 1], 'A', grid[row + 1][col + 1]]
                    .into_iter()
                    .collect();
                let diag2: String = [grid[row - 1][col + 1], 'A', grid[row + 1][col - 1]]
                    .into_iter()
                    .collect();

                if target.contains(&diag1) && target.contains(&diag2) {
                    total += 1;
                }
            }
        }
    }

    total
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = parse_input(input);
    Some(num_xmas(&grid))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = parse_input(input);
    Some(num_cross_mas(&grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
