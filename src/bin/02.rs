advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_input(input);

    Some(data.iter()
        .map(|row| if input_valid(row) {1} else {0})
        .sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_input(input);

    Some(data.iter()
        .map(|row| if (0..row.len())
            .find(|i| {
                let mut row = row.clone();
                row.remove(*i);
                input_valid(&row)
            }).is_some() {1} else {0})
        .sum::<u32>())
}

fn input_valid(input: &Vec<i32>) -> bool {
    input.windows(3)
        .find(|item| {
            let difference1 = item[0] - item[1];
            if difference1.abs() < 1 || difference1.abs() > 3 {
                return true;
            }
            let difference2 = item[1] - item[2];
            if difference2.abs() < 1 || difference2.abs() > 3 {
                return true;
            }
            if (difference1 > 0) != (difference2 > 0) {
                return true;
            }
            false
        }).is_none()
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().expect("Could not parse number"))
                .collect()
        })
        .collect()
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
