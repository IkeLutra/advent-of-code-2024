fn parse_to_levels(input: String) -> Vec<Vec<usize>> {
    input
        .split('\n')
        .map(|line| {
            line.split_whitespace()
                .map(|i| str::parse::<usize>(i).unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(level: Vec<usize>) -> bool {
    let decreasing = level[0] > level[1];
    for w in level.windows(2) {
        if w[0].abs_diff(w[1]) < 1 || w[0].abs_diff(w[1]) > 3 {
            return false;
        }
        if decreasing && w[1] > w[0] {
            return false;
        }
        if !decreasing && w[1] < w[0] {
            return false;
        }
    }
    true
}

fn is_safe_with_dampener(level: Vec<usize>) -> bool {
    if !is_safe(level.clone()) {
        for i in 0..level.len() {
            let slice = [&level[0..i], &level[i + 1..level.len()]].concat();
            if is_safe(slice) {
                return true;
            }
        }
        return false;
    }
    true
}

pub fn check_levels_safe(input: String, dampener: bool) -> usize {
    let checker = if dampener {
        is_safe_with_dampener
    } else {
        is_safe
    };
    let levels = parse_to_levels(input);
    levels
        .into_iter()
        .map(|level| if checker(level) { 1 } else { 0 })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day_2::{check_levels_safe, is_safe, is_safe_with_dampener, parse_to_levels};

    #[test]
    fn test_parse_to_levels() {
        let input: String = fs::read_to_string("./src/day_2/example.txt").unwrap();
        assert_eq!(
            parse_to_levels(input),
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9]
            ]
        )
    }

    #[test]
    fn test_is_safe() {
        assert_eq!(is_safe(vec![7, 6, 4, 2, 1]), true);
        assert_eq!(is_safe(vec![1, 2, 7, 8, 9]), false);
        assert_eq!(is_safe(vec![9, 7, 6, 2, 1]), false);
        assert_eq!(is_safe(vec![1, 3, 2, 4, 5]), false);
        assert_eq!(is_safe(vec![8, 6, 4, 4, 1]), false);
        assert_eq!(is_safe(vec![1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn test_is_safe_with_dampener() {
        assert_eq!(is_safe_with_dampener(vec![7, 6, 4, 2, 1]), true);
        assert_eq!(is_safe_with_dampener(vec![1, 2, 7, 8, 9]), false);
        assert_eq!(is_safe_with_dampener(vec![9, 7, 6, 2, 1]), false);
        assert_eq!(is_safe_with_dampener(vec![1, 3, 2, 4, 5]), true);
        assert_eq!(is_safe_with_dampener(vec![8, 6, 4, 4, 1]), true);
        assert_eq!(is_safe_with_dampener(vec![1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn test_check_levels_safe_with_dampener() {
        let input: String = fs::read_to_string("./src/day_2/input.txt").unwrap();
        assert_eq!(check_levels_safe(input, true), 364)
    }
}
