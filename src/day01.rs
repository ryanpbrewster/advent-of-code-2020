#[cfg(test)]
mod test {
    use itertools::Itertools;

    #[test]
    fn small1() {
        let input = r#"
            1721
            979
            366
            299
            675
            1456
        "#;
        let entries: Vec<i32> = input
            .split_ascii_whitespace()
            .map(|token| token.parse::<i32>().unwrap())
            .collect();

        let (a, b) = entries
            .iter()
            .tuple_combinations()
            .find(|(&a, &b)| a + b == 2020)
            .unwrap();
        assert_eq!(a * b, 514579);
    }

    #[test]
    fn normal1() {
        let input = std::fs::read_to_string("data/day01.input").unwrap();
        let entries: Vec<i32> = input
            .split_ascii_whitespace()
            .map(|token| token.parse::<i32>().unwrap())
            .collect();
        let (a, b) = entries
            .iter()
            .tuple_combinations()
            .find(|(&a, &b)| a + b == 2020)
            .unwrap();
        assert_eq!(a * b, 996075);
    }

    #[test]
    fn normal2() {
        let input = std::fs::read_to_string("data/day01.input").unwrap();
        let entries: Vec<i32> = input
            .split_ascii_whitespace()
            .map(|token| token.parse::<i32>().unwrap())
            .collect();
        let (a, b, c) = entries
            .iter()
            .tuple_combinations()
            .find(|(&a, &b, &c)| a + b + c == 2020)
            .unwrap();
        assert_eq!(a * b * c, 51810360);
    }
}
