use regex::Regex;

// TODO: Understand lifetimes so you could pass an iterator here
fn parse_numbers(input: String) -> Vec<(usize, usize)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(&input)
        .map(|c| c.extract::<2>())
        .map(|(_, [x, y])| (str::parse(x).unwrap(), str::parse(y).unwrap()))
        .collect()
}

fn parse_instructions(input: String) -> Vec<(usize, usize)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();
    let mut values: Vec<(usize, usize)> = vec![];
    let mut enabled = true;
    for captures in re.captures_iter(&input) {
        let full = captures.get(0).unwrap().as_str();
        if full.starts_with("mul") && enabled {
            values.push((
                str::parse(captures.get(1).unwrap().as_str()).unwrap(),
                str::parse(captures.get(2).unwrap().as_str()).unwrap(),
            ))
        } else if full == "don't()" {
            enabled = false;
        } else if full == "do()" {
            enabled = true;
        }
    }
    values
}

fn calculate_total(input: Vec<(usize, usize)>) -> usize {
    input.into_iter().map(|(x, y)| x * y).sum()
}

pub fn decode_memory(input: String) -> usize {
    calculate_total(parse_numbers(input))
}

pub fn decode_memory_full(input: String) -> usize {
    calculate_total(parse_instructions(input))
}

#[cfg(test)]
mod tests {
    use crate::day_3::{calculate_total, decode_memory, parse_instructions, parse_numbers};
    #[test]
    fn test_parse_numbers() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(
            parse_numbers(input.to_string()),
            vec![(2, 4), (5, 5), (11, 8), (8, 5)]
        )
    }

    #[test]
    fn test_calculate_total() {
        let input = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        assert_eq!(calculate_total(input), 161)
    }

    #[test]
    fn test_decode_memory() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(decode_memory(input.to_string()), 161)
    }

    #[test]
    fn test_parse_instructions() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(parse_instructions(input.to_string()), vec![(2, 4), (8, 5)])
    }
}
