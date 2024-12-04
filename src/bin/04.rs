use strum::{EnumIter, IntoEnumIterator};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    Some(matrix.iter()
        .enumerate()
        .map(|(i, line)| {
             line.iter()
                 .enumerate()
                 .filter(|(_, c)| **c == 'X')
                 .map(|(j, _)| find_xmas(i, j, &matrix))
                 .sum::<u32>()
        })
        .sum())
}

#[derive(Debug, Clone, Copy, EnumIter)]
enum Direction {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

fn find_xmas(i: usize, j: usize, matrix: &Vec<Vec<char>>) -> u32 {
    let letters = vec!['M', 'A', 'S'];
    Direction::iter()
        .map(|direction| {
            let mut location = (i, j);
            for letter in &letters {
                if let Some(new_loc) = is_letter_match(*letter, direction, location.0, location.1, matrix) {
                    location = new_loc;
                } else {
                    return 0;
                }
            }
            1
        })
        .sum()
}

fn is_letter_match(letter: char, direction: Direction, i: usize, j: usize, matrix: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let i = i as i32;
    let j = j as i32;
    let (x, y) = match direction {
        Direction::Top => (i-1, j),
        Direction::TopRight => (i-1, j+1),
        Direction::Right => (i, j+1),
        Direction::BottomRight => (i+1, j+1),
        Direction::Bottom => (i+1, j),
        Direction::BottomLeft => (i+1, j-1),
        Direction::Left => (i, j-1),
        Direction::TopLeft => (i-1, j-1),
    };

    if x < 0 || x >= matrix.len() as i32 {
        return None;
    }
    if y < 0 || y >= matrix[x as usize].len() as i32 {
        return None;
    }
    if matrix[x as usize][y as usize] != letter {
        return None;
    }
    Some((x as usize, y as usize))
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    Some(matrix.iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter(|(j, c)| **c == 'A' && i >= 1 && i+1 < matrix.len() && *j >= 1 && *j+1 < matrix[i].len())
                .map(|(j, _)| {
                    if ((matrix[i-1][j-1] == 'M' && matrix[i+1][j+1] == 'S')
                        || (matrix[i-1][j-1] == 'S' && matrix[i+1][j+1] == 'M'))
                        && ((matrix[i+1][j-1] == 'M' && matrix[i-1][j+1] == 'S')
                        || (matrix[i+1][j-1] == 'S' && matrix[i-1][j+1] == 'M')) {1} else {0}
                })
                .sum::<u32>()
        })
        .sum())
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
