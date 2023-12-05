use advent_of_code::read_lines;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{eof, value};
use nom::multi::many_till;
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
        .map(|s| s.chars().filter(|c| c.is_digit(10)).collect())
        .map(|s: String| {
            let mut characters = s.chars();
            let first = characters.next().unwrap();
            let last = characters.last();
            let mut res = String::from(first);
            match last {
                Some(l) => res.push(l),
                None => res.push(first),
            }

            res.parse::<u32>().unwrap()
        })
        .sum()
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Expr {
    Null,
    Num(u32),
}

impl Expr {
    fn to_number(&self) -> Option<u32> {
        match self {
            Expr::Null => None,
            Expr::Num(x) => Some(*x),
        }
    }
}

fn digit_parser(input: &str) -> IResult<&str, Expr> {
    let mut characters = input.chars();
    let first = characters.next();
    let rest = characters.as_str();
    match first {
        Some(c) => {
            if c.is_ascii_digit() {
                Ok((rest, Expr::Num(c.to_digit(10).unwrap())))
            } else {
                Err(nom::Err::Error(nom::error::Error {
                    input,
                    code: nom::error::ErrorKind::Digit,
                }))
            }
        }
        None => Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Digit,
        })),
    }
}

fn token_parser(input: &str) -> IResult<&str, Expr> {
    alt((
        value(Expr::Num(1), tag("one")),
        value(Expr::Num(2), tag("two")),
        value(Expr::Num(3), tag("three")),
        value(Expr::Num(4), tag("four")),
        value(Expr::Num(5), tag("five")),
        value(Expr::Num(6), tag("six")),
        value(Expr::Num(7), tag("seven")),
        value(Expr::Num(8), tag("eight")),
        value(Expr::Num(9), tag("nine")),
    ))(input)
}

fn modified_token_parser(input: &str) -> IResult<&str, Expr> {
    let tags: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let cloned_tags = tags.clone();

    let matched_tag = tags
        .into_iter()
        .filter(|m| input.starts_with(m))
        .collect::<Vec<&str>>()
        .pop();

    match matched_tag {
        Some(t) => {
            let matched_tag_val: u32 = cloned_tags
                .into_iter()
                .position(|m| m == t)
                .unwrap()
                .try_into()
                .unwrap();

            let mut characters = input.chars();
            let _ = characters.next();
            let rest = characters.as_str();
            Ok((rest, Expr::Num(matched_tag_val + 1)))
        }
        None => Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Tag,
        })),
    }
}

fn character_consumer(input: &str) -> IResult<&str, Expr> {
    let mut characters = input.chars();
    let first = characters.next();
    let rest = characters.as_str();
    match first {
        Some(c) => {
            if c.is_ascii_digit() {
                Ok((rest, Expr::Num(c.to_digit(10).unwrap())))
            } else {
                Ok((rest, Expr::Null))
            }
        }
        None => Ok(("", Expr::Null)),
    }
}

fn combined_parser(input: &str) -> IResult<&str, Expr> {
    alt((modified_token_parser, digit_parser, character_consumer))(input)
}

fn second_case(input: Vec<String>) -> u32 {
    input
        .into_iter()
        .map(|i: String| {
            let istr = i.to_owned();
            let (_, (parsed_expr, _)) = many_till(combined_parser, eof)(&istr).unwrap();
            let mut filtered_parsed_expr: Vec<Expr> = parsed_expr
                .into_iter()
                .filter(|e: &Expr| match e {
                    Expr::Num(_) => true,
                    Expr::Null => false,
                })
                .collect();

            let num = match filtered_parsed_expr.len() {
                0 => panic!("Crap"),
                1 => {
                    let first = filtered_parsed_expr.pop().unwrap().to_number().unwrap();
                    first * 10 + first
                }
                _ => {
                    let first = filtered_parsed_expr[0].to_number().unwrap();
                    let last = filtered_parsed_expr.pop().unwrap().to_number().unwrap();
                    first * 10 + last
                }
            };

            return num;
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn satisfies_first_case() {
        let test_input: Vec<String> = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .into_iter()
            .map(|s| String::from(s))
            .collect();

        let result = first_case(test_input);
        assert_eq!(result, 142);
    }

    #[test]
    fn satisfied_second_case() {
        let test_input: Vec<String> = [
            "two65eightbkgqcsn91qxkfvg",
            "neightwompstbkqv1fourfthdcfgtrkqzgrbfrczxbdn",
            "43qsrrlxxq",
            "898dbpjmdqjgtrvdvlxxdnvlfhncdzrt",
            "jninedsrvftdlcg4hhztwofourskrjhcjvthree",
            "five562 ",
        ]
        .into_iter()
        .map(|s| String::from(s))
        .collect();

        let result = second_case(test_input);
        assert_eq!(result, 282);
    }

    #[test]
    fn digit_parser_test() {
        let test_input: &str = "1abc2";
        let result = digit_parser(test_input);

        assert_eq!(result, Ok(("abc2", Expr::Num(1))));
    }

    #[test]
    fn digit_parser_test_fail() {
        let test_input: &str = "abc2";
        let result = digit_parser(test_input);

        let error = Err(nom::Err::Error(nom::error::Error {
            input: "abc2",
            code: nom::error::ErrorKind::Digit,
        }));

        assert_eq!(result, error)
    }

    #[test]
    fn token_parser_test() {
        let test_input: &str = "one1abc2";
        let result = token_parser(test_input);

        assert_eq!(result, Ok(("1abc2", Expr::Num(1))));
    }

    #[test]
    fn token_parser_test_fail() {
        let test_input: &str = "abc2";
        let result = token_parser(test_input);

        let error = Err(nom::Err::Error(nom::error::Error {
            input: "abc2",
            code: nom::error::ErrorKind::Tag,
        }));

        assert_eq!(result, error)
    }

    #[test]
    fn combined_parser_test() {
        let test_input: &str = "one1abc2";
        let result = combined_parser(test_input);
        assert_eq!(result, Ok(("1abc2", Expr::Num(1))));

        let test_input: &str = "1abc2";
        let result = combined_parser(test_input);
        assert_eq!(result, Ok(("abc2", Expr::Num(1))));

        let test_input: &str = "abc2";
        let result = combined_parser(test_input);
        assert_eq!(result, Ok(("bc2", Expr::Null)));
    }

    #[test]
    fn repeat_parser_test() {
        let test_input: &str = "onetwoa";
        let result = many_till(combined_parser, eof)(test_input);
        assert_eq!(
            result,
            Ok(("", (vec![Expr::Num(1), Expr::Num(2), Expr::Null], "")))
        );
    }

    #[test]
    fn repeat_parser_test_1() {
        let test_input: &str = "one1abc2";
        let result = many_till(combined_parser, eof)(test_input);
        assert_eq!(
            result,
            Ok((
                "",
                (
                    vec![
                        Expr::Num(1),
                        Expr::Num(1),
                        Expr::Null,
                        Expr::Null,
                        Expr::Null,
                        Expr::Num(2)
                    ],
                    ""
                )
            ))
        );
    }

    #[test]
    fn repeat_parser_test_2() {
        let test_input: &str = "1a2";
        let result = many_till(combined_parser, eof)(test_input);
        assert_eq!(
            result,
            Ok(("", (vec![Expr::Num(1), Expr::Null, Expr::Num(2)], "")))
        );
    }

    #[test]
    fn repeat_parser_test_3() {
        let test_input: &str = "a2";
        let result = many_till(combined_parser, eof)(test_input);
        assert_eq!(result, Ok(("", (vec![Expr::Null, Expr::Num(2)], ""))));
    }
}
