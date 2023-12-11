use advent_of_code::read_lines;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{digit1, space0};
use nom::multi::many0;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

fn main() {
    let input = read_lines();
    let x = first_case(input);
    println!("Output 1st: {}", x);

    let input = read_lines();
    let x = second_case(input);
    println!("Output 2nd: {}", x);
}

fn first_case(input: Vec<String>) -> u32 {
    input
        .into_iter()
        .map(|s| {
            let i = s.to_owned();
            match line_parser(&i) {
                Ok((_, (numbers, winners))) => {
                    let matches = numbers.iter().filter(|n| winners.contains(n)).count();
                    let power = u32::try_from(matches).unwrap();
                    let base: u32 = 2;
                    if power > 0 {
                        println!("{} => {}", i, base.pow(power - 1));
                        base.pow(power - 1)
                    } else {
                        0
                    }
                }
                Err(_) => 0,
            }
        })
        .sum()
}

fn second_case(input: Vec<String>) -> u32 {
    let length = input.len();
    let mut scratch_card_counts: Vec<u32> = vec![1; length];
    let mut scratch_card_wins: Vec<u32> = vec![1; length];

    let _: Vec<usize> = input
        .into_iter()
        .enumerate()
        .map(|(index, s): (usize, String)| {
            let i = s.to_owned();
            match line_parser(&i) {
                Ok((_, (numbers, winners))) => {
                    let matches = numbers.iter().filter(|n| winners.contains(n)).count();
                    if matches > 0 {
                        let parent_card_count = scratch_card_counts[index];
                        for idx in (index + 1)..(index + matches + 1) as usize {
                            let current_count = &mut scratch_card_counts[idx];
                            *current_count += 1 * parent_card_count;
                        }

                        let current_wins = &mut scratch_card_wins[index];
                        *current_wins = u32::try_from(matches).unwrap();
                    }

                    matches
                }
                Err(e) => {
                    println!("Something went wrong, {:?}", e);
                    0
                }
            }
        })
        .collect();

    scratch_card_counts.into_iter().sum()
}

fn line_parser(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    preceded(
        terminated(take_until(":"), tag(":")),
        separated_pair(numbers_parser, tag("|"), numbers_parser),
    )(input)
}

fn numbers_parser(input: &str) -> IResult<&str, Vec<&str>> {
    many0(preceded(space0, terminated(digit1, space0)))(input)
}

#[cfg(test)]
mod day4_tests {
    use super::*;

    #[test]
    fn test_line_parser() {
        let result = line_parser("Card 1: 43 45 | 12 45");
        assert_eq!(result, Ok(("", (vec!["43", "45"], vec!["12", "45"]))))
    }

    #[test]
    fn test_second_case() {
        let input = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ];

        let result = second_case(input);
        assert_eq!(result, 30)
    }
}
