use itertools::Itertools;
use std::collections::VecDeque;

fn first_flaw(xs: &[u64], buffer_size: usize) -> Option<u64> {
    let mut buffer = VecDeque::new();
    for &x in &xs[..buffer_size] {
        buffer.push_back(x);
    }
    for &x in &xs[buffer_size..] {
        if !buffer.iter().tuple_combinations().any(|(a, b)| a + b == x) {
            return Some(x);
        }
        buffer.pop_front();
        buffer.push_back(x);
    }
    None
}

fn contiguous_region(xs: &[u64], target: u64) -> Option<&[u64]> {
    assert!(target > 0);
    let mut lo = 0;
    let mut acc = 0;
    for hi in 0..xs.len() {
        acc += xs[hi];
        while acc > target {
            acc -= xs[lo];
            lo += 1;
        }
        if acc == target {
            return Some(&xs[lo..=hi]);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::{contiguous_region, first_flaw};

    const SMALL: &str = r"
        35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576
    ";

    #[test]
    fn small1() {
        let input: Vec<u64> = SMALL
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(first_flaw(&input, 5).unwrap(), 127);
    }

    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day09.input").unwrap();
        let input: Vec<u64> = raw
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(first_flaw(&input, 25).unwrap(), 375054920);
    }

    #[test]
    fn small2() {
        let input: Vec<u64> = SMALL
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let target = first_flaw(&input, 5).unwrap();
        let region = contiguous_region(&input, target).unwrap();
        let min = *region.iter().min().unwrap();
        let max = *region.iter().max().unwrap();
        assert_eq!(min + max, 62);
    }

    #[test]
    fn normal2() {
        let raw = std::fs::read_to_string("data/day09.input").unwrap();
        let input: Vec<u64> = raw
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let target = first_flaw(&input, 25).unwrap();
        let region = contiguous_region(&input, target).unwrap();
        let min = *region.iter().min().unwrap();
        let max = *region.iter().max().unwrap();
        assert_eq!(min + max, 54142584);
    }
}
