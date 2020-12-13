struct Schedule {
    t0: i32,
    ts: Vec<i32>,
}

fn solve1(input: Schedule) -> i32 {
    let (id, t) = input
        .ts
        .iter()
        .map(|&id| (id, id - (input.t0 % id)))
        .min_by_key(|&(_id, t)| t)
        .unwrap();
    id * t
}
#[cfg(test)]
mod test {
    use super::{solve1, Schedule};

    const SMALL: &str = r"
        939
        7,13,x,x,59,x,31,19
    ";
    fn parse(input: &str) -> Schedule {
        let mut lines = input.trim().lines().map(|l| l.trim());
        let t0 = lines.next().unwrap().parse().unwrap();
        let ts = lines
            .next()
            .unwrap()
            .split(',')
            .filter(|&t| t != "x")
            .map(|t| t.parse().unwrap())
            .collect();
        Schedule { t0, ts }
    }
    #[test]
    fn small1() {
        let input = parse(SMALL);
        assert_eq!(solve1(input), 295);
    }

    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day13.input").unwrap();
        let input = parse(&raw);
        assert_eq!(solve1(input), 153);
    }
}
