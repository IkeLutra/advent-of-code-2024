fn convert_to_lists(data: String) -> (Vec<usize>, Vec<usize>) {
    let ints: Vec<Vec<usize>> = data
        .split('\n')
        .map(|d| {
            d.split_whitespace()
                .map(|s| str::parse::<usize>(s).unwrap())
                .collect()
        })
        .collect();
    let mut first: Vec<usize> = ints.iter().map(|d| d[0]).collect();
    let mut second: Vec<usize> = ints.iter().map(|d| d[1]).collect();
    first.sort();
    second.sort();
    (first, second)
}
// Puzzle 1
pub fn calculate_distance(data: String) -> usize {
    let (first, second) = convert_to_lists(data);
    first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| {
            return a.abs_diff(*b);
        })
        .sum()
}

pub fn calculate_simularity_score(data: String) -> usize {
    let (left, right) = convert_to_lists(data);
    left.into_iter()
        .map(|d| {
            d * (right
                .iter()
                .filter(|x| **x == d)
                .collect::<Vec<&usize>>()
                .len())
        })
        .sum()
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::day_1::{calculate_distance, calculate_simularity_score};

    #[test]
    fn test_calculate_distance() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(calculate_distance(input.to_string()), 11);
    }

    #[test]
    fn test_calculate_distance_real() {
        let input = fs::read_to_string("./src/day_1/input.txt").expect("must be able to read file");
        assert_eq!(calculate_distance(input), 1765812);
    }

    #[test]
    fn test_simularity_score() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(calculate_simularity_score(input.to_string()), 31);
    }
}
