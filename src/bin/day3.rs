use std::usize;

use advent_of_code::read_file_as_one_str;
use nom::branch::alt;
use nom::bytes::complete::{is_a, take_while};
use nom::character::complete::digit1;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::sequence::preceded;
use nom::IResult;
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;
const LINE_WIDTH: usize = 140;

fn main() {
    let input: String = read_file_as_one_str();
    let x = first_case(&input.to_owned());
    println!("Output 1st: {}", x);

    println!("Relevant indices {:?}", relevant_indices(141, 5))
}

fn first_case(input: &str) -> usize {
    let span = Span::new(input);
    match combined_parser(span) {
        Ok((_, (spans, _))) => {
            println!("{:?}", spans);
            spans.len()
        }
        Err(e) => {
            println!("ERROR: {:?}", e);
            usize::MIN
        }
    }
}

fn analyze_spans(spans: Vec<Span>) {
    spans
        .into_iter()
        .filter(|s: &Span| !s.fragment().contains('.'));
}

fn relevant_indices(offset: usize, length: usize) -> Vec<usize> {
    let idx_fn = |i: usize| -> Vec<usize> {
        let mut idxs = vec![
            i + 1,
            i + LINE_WIDTH - 1,
            i + LINE_WIDTH,
            i + LINE_WIDTH + 1,
        ];

        if (i - LINE_WIDTH - 1) > 0 {
            idxs.push(i - LINE_WIDTH - 1)
        }

        if (i - LINE_WIDTH - 1) > 0 {
            idxs.push(i - LINE_WIDTH)
        }
        if (i - LINE_WIDTH) > 0 {
            idxs.push(i - LINE_WIDTH)
        }
        if (i - LINE_WIDTH + 1) > 0 {
            idxs.push(i - LINE_WIDTH + 1)
        }
        if (i - 1) > 0 {
            idxs.push(i - 1)
        }
        idxs
    };

    let mut indices = Vec::new();
    let mut i = offset;

    while i < offset + length - 1 {
        let mut idxs = idx_fn(i);
        indices.append(&mut idxs);
        i += 1;
    }

    indices
}

fn dot_parser(input: Span) -> IResult<Span, Span> {
    take_while(|c| c == '.')(input)
}

fn number_parser(input: Span) -> IResult<Span, Span> {
    preceded(dot_parser, digit1)(input)
}

fn symbol_parser(input: Span) -> IResult<Span, Span> {
    preceded(dot_parser, is_a("\"!@#$%^&*()+=-_'\\|/?}{]["))(input)
}

fn combined_parser(input: Span) -> IResult<Span, (Vec<Span>, Span)> {
    many_till(alt((symbol_parser, number_parser, dot_parser)), eof)(input)
}

#[cfg(test)]
mod day3_tests {
    use super::*;
    use std::ops::Index;

    #[test]
    fn test_dot_parser() {
        let result = dot_parser(Span::new("....343."));
        let (rest, matched) = result.unwrap();
        assert_eq!(rest.fragment(), &"343.");
        assert_eq!(matched.fragment(), &"....");
    }

    #[test]
    fn test_number_parser() {
        let result = number_parser(Span::new("343...."));
        let (rest, matched) = result.unwrap();
        assert_eq!(rest.fragment(), &"....");
        assert_eq!(matched.fragment(), &"343");
    }

    #[test]
    fn test_symbol_parser() {
        let result = symbol_parser(Span::new("%.."));
        let (rest, matched) = result.unwrap();
        assert_eq!(rest.fragment(), &"..");
        assert_eq!(matched.fragment(), &"%");
    }

    #[test]
    fn test_line_parser() {
        let result = combined_parser(Span::new("..31%..#"));
        let (_, (matches, _)) = result.unwrap();
        let first_match = matches.index(0);
        assert_eq!(matches.len(), 3);
        assert_eq!(first_match.location_offset(), 2);
        assert_eq!(first_match.fragment(), &"31")
    }
}
