use advent_of_code::read_lines;
use nom::branch::alt;
use nom::bytes::complete::{is_a, tag};
use nom::character::complete::space0;
use nom::combinator::{eof, opt};
use nom::multi::{many0, many_till};
use nom::sequence::{terminated, tuple};
use nom::IResult;

fn main() {
    let input = read_lines();
    let x = first_case(input, 12, 13, 14);
    println!("Output 1st: {}", x);

    let input = read_lines();
    let x = second_case(input);
    println!("Output 1st: {}", x);
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Pick {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct GameData {
    id: u32,
    picks: Vec<Pick>,
    max_red: u32,
    max_blue: u32,
    max_green: u32,
}

fn first_case(input: Vec<String>, max_red: u32, max_green: u32, max_blue: u32) -> u32 {
    input
        .into_iter()
        .map(|i: String| {
            let inp = i.to_owned();
            match line_parser(&inp) {
                Ok((_, gd)) => gd,
                Err(_) => GameData {
                    id: 0,
                    picks: vec![],
                    max_red: 0,
                    max_blue: 0,
                    max_green: 0,
                },
            }
        })
        .map(|gd: GameData| {
            if gd.max_red > max_red || gd.max_blue > max_blue || gd.max_green > max_green {
                0
            } else {
                gd.id
            }
        })
        .sum()
}

fn second_case(input: Vec<String>) -> u32 {
    input
        .into_iter()
        .map(|i: String| {
            let inp = i.to_owned();
            match line_parser(&inp) {
                Ok((_, gd)) => gd,
                Err(_) => GameData {
                    id: 0,
                    picks: vec![],
                    max_red: 0,
                    max_blue: 0,
                    max_green: 0,
                },
            }
        })
        .map(|gd: GameData| gd.max_red * gd.max_green * gd.max_blue)
        .sum()
}

fn number_parser(input: &str) -> IResult<&str, &str> {
    is_a("1234567890")(input)
}

fn game_id_parser(input: &str) -> IResult<&str, GameData> {
    let res = tuple((tag("Game"), space0, number_parser, is_a(":")))(input);
    match res {
        Ok((rest, (_, _, game_id, _))) => Ok((
            rest,
            GameData {
                id: game_id.parse::<u32>().unwrap(),
                picks: vec![],
                max_blue: 0,
                max_green: 0,
                max_red: 0,
            },
        )),
        Err(e) => Err(e),
    }
}

fn pick_parser(input: &str) -> IResult<&str, Pick> {
    let res = terminated(
        many0(tuple((
            space0,
            number_parser,
            space0,
            alt((tag("red"), tag("blue"), tag("green"))),
            opt(tag(",")),
        ))),
        opt(tag(";")),
    )(input);

    match res {
        Ok((rest, pv)) => {
            let init: Pick = Pick {
                red: 0,
                blue: 0,
                green: 0,
            };
            let pick = pv
                .into_iter()
                .fold(init, |mut accumulator, (_, num, _, color, _)| match color {
                    "red" => {
                        accumulator.red = num.parse::<u32>().unwrap();
                        accumulator
                    }
                    "blue" => {
                        accumulator.blue = num.parse::<u32>().unwrap();
                        return accumulator;
                    }
                    "green" => {
                        accumulator.green = num.parse::<u32>().unwrap();
                        return accumulator;
                    }
                    _ => panic!("Shouldn't get any color apart from RGB"),
                });
            Ok((rest, pick))
        }
        Err(e) => Err(e),
    }
}

fn repeated_pick_parser(input: &str) -> IResult<&str, (Vec<Pick>, &str)> {
    many_till(pick_parser, eof)(input)
}

fn line_parser(input: &str) -> IResult<&str, GameData> {
    match game_id_parser(input) {
        Ok((rest, mut game)) => match repeated_pick_parser(rest) {
            Ok((r, (picks, _))) => {
                let cloned_picks = picks.clone();
                let (max_red, max_green, max_blue) =
                    picks.into_iter().fold((0, 0, 0), |(r, g, b), pick| {
                        let n_red = if pick.red > r { pick.red } else { r };
                        let n_green = if pick.green > g { pick.green } else { g };
                        let n_blue = if pick.blue > b { pick.blue } else { b };
                        (n_red, n_green, n_blue)
                    });

                game.picks = cloned_picks;
                game.max_red = max_red;
                game.max_blue = max_blue;
                game.max_green = max_green;
                return Ok((r, game));
            }
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod day2_tests {
    use super::*;

    #[test]
    fn test_game_id_parser() {
        let result = game_id_parser("Game 1:");
        assert_eq!(
            result,
            Ok((
                "",
                GameData {
                    id: 1,
                    picks: vec![],
                    max_red: 0,
                    max_green: 0,
                    max_blue: 0
                }
            ))
        );
    }

    #[test]
    fn test_pick_parser() {
        let result = pick_parser(" 3 blue, 4 red; 2 green");
        assert_eq!(
            result,
            Ok((
                " 2 green",
                Pick {
                    red: 4,
                    green: 0,
                    blue: 3
                }
            ))
        );
    }

    #[test]
    fn test_repeat_pick_parser() {
        let result = repeated_pick_parser(" 3 blue, 4 red; 2 green; 1 red, 2 blue, 3 green");
        assert_eq!(
            result,
            Ok((
                "",
                (
                    vec![
                        Pick {
                            red: 4,
                            green: 0,
                            blue: 3
                        },
                        Pick {
                            green: 2,
                            red: 0,
                            blue: 0
                        },
                        Pick {
                            red: 1,
                            blue: 2,
                            green: 3,
                        }
                    ],
                    ""
                )
            ))
        );
    }

    #[test]
    fn test_line_parser() {
        let result = line_parser("Game 1: 3 blue, 4 red; 2 green");
        assert_eq!(
            result,
            Ok((
                "",
                GameData {
                    id: 1,
                    max_green: 2,
                    max_blue: 3,
                    max_red: 4,
                    picks: vec![
                        Pick {
                            red: 4,
                            blue: 3,
                            green: 0
                        },
                        Pick {
                            green: 2,
                            red: 0,
                            blue: 0
                        }
                    ]
                }
            ))
        )
    }

    #[test]
    fn test_first_case() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 2 green".to_string(),
            "Game 2: 100 blue; 4 red".to_string(),
        ];
        let result = first_case(input, 5, 5, 5);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_second_case() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];
        let result = second_case(input);
        assert_eq!(result, 2286);
    }
}
