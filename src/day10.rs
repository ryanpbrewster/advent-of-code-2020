use std::collections::HashMap;

fn tally_diffs(xs: &[u32]) -> HashMap<u32, usize> {
    let mut sorted = xs.to_vec();
    sorted.sort();

    let mut counts = HashMap::new();
    for ab in sorted.windows(2) {
        *counts.entry(ab[1] - ab[0]).or_default() += 1;
    }
    counts
}

fn count_arrangements(xs: &[u32]) -> u64 {
    let mut sorted = xs.to_vec();
    sorted.sort();

    let mut counts = vec![0u64; *sorted.last().unwrap() as usize + 1];
    counts[0] = 1;
    let mut cur = 1;
    for i in 1..counts.len() {
        if sorted[cur] as usize == i {
            counts[i] = match i {
                1 => 1,
                2 => counts[0] + counts[1],
                _ => counts[i - 3] + counts[i - 2] + counts[i - 1],
            };
            cur += 1;
        }
    }
    *counts.last().unwrap()
}
#[cfg(test)]
mod test {
    use super::{count_arrangements, tally_diffs};

    const TINY: &str = r"16 10 15 5 1 11 7 19 6 12 4";
    const SMALL: &str = r"
        28 33 18 42 31 14 46 20 48 47 24 23 49 45 19
        38 39 11 1 32 25 35 8 17 7 9 4 2 34 10 3
    ";
    fn parse(input: &str) -> Vec<u32> {
        let mut real: Vec<u32> = input
            .split_ascii_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        // The socket is effectively 0.
        real.push(0);
        // The device is effectively max(adapters) + 3
        real.push(real.iter().max().unwrap() + 3);
        real
    }

    #[test]
    fn small1() {
        assert_eq!(
            tally_diffs(&parse(TINY)),
            vec![(1, 7), (3, 5)].into_iter().collect()
        );
        assert_eq!(
            tally_diffs(&parse(SMALL)),
            vec![(1, 22), (3, 10)].into_iter().collect()
        );
    }

    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day10.input").unwrap();
        let diffs = tally_diffs(&parse(&raw));
        assert_eq!(diffs[&1] * diffs[&3], 2100);
    }

    #[test]
    fn small2() {
        assert_eq!(count_arrangements(&parse(TINY)), 8);
        assert_eq!(count_arrangements(&parse(SMALL)), 19208);
    }

    #[test]
    fn normal2() {
        let raw = std::fs::read_to_string("data/day10.input").unwrap();
        assert_eq!(count_arrangements(&parse(&raw)), 16198260678656);
    }
}
