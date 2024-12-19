use itertools::Itertools;
use std::collections::BTreeMap;

fn parse_input(input: String) -> (BTreeMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let mut rules: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut sequences: Vec<Vec<usize>> = vec![];
    let mut is_rules = true;
    for line in input.lines() {
        if line.is_empty() {
            is_rules = false;
            continue;
        }
        if is_rules {
            if let Some((key, value)) = line
                .split("|")
                .map(|v| v.trim().parse::<usize>().unwrap())
                .collect_tuple()
            {
                if let Some(values) = rules.get_mut(&key) {
                    values.push(value);
                } else {
                    rules.insert(key, vec![value]);
                }
            }
        } else {
            sequences.push(
                line.split(",")
                    .map(|v| v.trim().parse::<usize>())
                    .flatten()
                    .collect(),
            )
        }
    }
    (rules, sequences)
}

fn is_correct_order(rules: &BTreeMap<usize, Vec<usize>>, sequence: Vec<usize>) -> bool {
    for (i, current) in sequence.iter().enumerate() {
        if let Some(specific_rules) = rules.get(current) {
            if specific_rules.iter().any(|v| {
                return sequence[0..i].contains(v);
            }) {
                return false;
            }
        }
    }
    true
}

pub fn check_puzzle_5(input: String) -> usize {
    let (rules, sequences) = parse_input(input);
    sequences
        .into_iter()
        .filter(|v| is_correct_order(&rules, v.to_vec()))
        .map(|v| v[v.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_parse_example() {
        let input = fs::read_to_string("src/day_5/example.txt")
            .expect("Should have been able to read the file");
        let result: (BTreeMap<usize, Vec<usize>>, Vec<Vec<usize>>) = super::parse_input(input);
        insta::assert_yaml_snapshot!(result.0);
        insta::assert_yaml_snapshot!(result.1);
    }
    #[test]
    fn test_is_correct_order() {
        let input = fs::read_to_string("src/day_5/example.txt")
            .expect("Should have been able to read the file");
        let (rules, sequences) = super::parse_input(input);
        let result = super::is_correct_order(&rules, sequences[0].clone());
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_correct_order_failure() {
        let input = fs::read_to_string("src/day_5/example.txt")
            .expect("Should have been able to read the file");
        let (rules, sequences) = super::parse_input(input);
        let result = super::is_correct_order(&rules, sequences[3].clone());
        assert_eq!(result, false);
    }

    #[test]
    fn test_check_puzzle_5() {
        let input = fs::read_to_string("src/day_5/example.txt")
            .expect("Should have been able to read the file");
        let result = super::check_puzzle_5(input);
        assert_eq!(result, 143);
    }
}
