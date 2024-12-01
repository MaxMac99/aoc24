advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_list, mut right_list) = parse_input(input);

    left_list.sort();
    right_list.sort();

    Some(left_list.iter()
        .zip(right_list.iter())
        .map(|(left, right)| (right - left).abs() as u32)
        .sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_list, right_list) = parse_input(input);

    Some(left_list.iter()
        .map(|left| right_list.clone().into_iter()
            .filter(|right| left.eq(right))
            .collect::<Vec<i32>>()
            .len() as i32 * left)
        .sum::<i32>() as u32)
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    input.lines()
        .map(|line| {
            let entries = line.split_whitespace()
                .map(|s| s.parse::<i32>().expect("Could not parse number"))
                .collect::<Vec<i32>>();
            (entries[0], entries[1])
        })
        .for_each(|(left, right)| {
            left_list.push(left);
            right_list.push(right);
        });
    (left_list, right_list)
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
