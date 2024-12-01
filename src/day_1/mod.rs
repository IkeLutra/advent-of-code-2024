// Puzzle 1
pub fn calculate_distance(data: String) -> isize {
    let ints: Vec<Vec<isize>> = data
        .split('\n')
        .map(|d| {
            d.split_whitespace()
                .map(|s| str::parse::<isize>(s).unwrap())
                .collect()
        })
        .collect();
    let mut first: Vec<isize> = ints.iter().map(|d| d[0]).collect();
    let mut second: Vec<isize> = ints.iter().map(|d| d[1]).collect();
    first.sort();
    second.sort();

    first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| {
            return (b - a).abs();
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::day_1::calculate_distance;

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
}
