use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::char;
use nom::character::is_digit;
use nom::multi::many0;
use nom::sequence::{delimited, separated_pair};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let (_, items) = many0(parse_val)(input).unwrap();
    Some(items.iter().flatten().map(|(l, r)| l * r).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, items) = many0(parse_till_dont)(input).unwrap();
    Some(items.iter().flatten().map(|(l, r)| l * r).sum())
}

fn parse_till_dont(input: &str) -> nom::IResult<&str, Vec<(u32, u32)>> {
    if input == "" {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Eof)));
    }
    let (rest, input) = take_until::<&str, &str, nom::error::Error<&str>>("don't()")(input)
        .map(|(rest, input)| (tag::<&str, &str, nom::error::Error<&str>>("don't()")(rest).unwrap().0, input))
        .unwrap_or_else(|_| ("", input));
    let (_, items) = many0(parse_val)(input)?;
    let rest = match take_until::<&str, &str, nom::error::Error<&str>>("do()")(rest) {
        Ok((rest, _)) => rest,
        Err(_) => return Ok(("", items.into_iter().flatten().collect())),
    };
    let (rest, _) = tag("do()")(rest)?;
    Ok((rest, items.into_iter().flatten().collect()))
}

fn parse_val(input: &str) -> nom::IResult<&str, Option<(u32, u32)>> {
    let (input, _) = take_until("mul")(input)?;
    let (input, _) = tag("mul")(input)?;
    let inner_parser = separated_pair(take_while(is_char_digit), char::<&str, nom::error::Error<&str>>(','), take_while(is_char_digit));
    match delimited(char('('), inner_parser, char(')'))(input) {
        Ok((next, (l, r))) => Ok((next, Some((l.parse().unwrap(), r.parse().unwrap())))),
        Err(_) => Ok((input, None)),
    }
}

pub fn is_char_digit(chr: char) -> bool {
    chr.is_ascii() && is_digit(chr as u8)
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
