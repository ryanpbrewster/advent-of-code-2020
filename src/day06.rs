use std::collections::HashSet;

fn count_any(pass: &str) -> usize {
    pass.trim()
        .split("\n\n")
        .map(|group| {
            let any = group
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<HashSet<char>>();
            any.len()
        })
        .sum()
}

fn count_all(pass: &str) -> usize {
    pass.trim()
        .split("\n\n")
        .map(|group| {
            let mut individuals = group
                .lines()
                .map(|line| line.trim().chars().collect::<HashSet<char>>());
            let first = individuals.next().unwrap_or_default();
            let common = individuals.fold(first, |acc, x| {
                acc.intersection(&x).cloned().collect::<HashSet<char>>()
            });
            common.len()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::{count_all, count_any};

    const SMALL: &str = r"
        abc

        a
        b
        c

        ab
        ac

        a
        a
        a
        a

        b
    ";

    #[test]
    fn small1() {
        assert_eq!(count_any(SMALL), 11);
    }

    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day06.input").unwrap();
        assert_eq!(count_any(&raw), 6742);
    }

    #[test]
    fn small2() {
        assert_eq!(count_all(SMALL), 6);
    }

    #[test]
    fn normal2() {
        let raw = std::fs::read_to_string("data/day06.input").unwrap();
        assert_eq!(count_all(&raw), 3447);
    }
}
