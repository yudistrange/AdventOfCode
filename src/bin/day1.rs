use advent_of_code::read_lines;
fn main() {
    let input = read_lines();
    let x = first_case(input);

    println!("Output {}", x)
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
}
