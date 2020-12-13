use itertools::Itertools;
struct Schedule {
    lo: i64,
    ts: Vec<Option<i64>>,
}

fn solve1(input: Schedule) -> i64 {
    let (id, t) = input
        .ts
        .iter()
        .filter_map(|id| id.as_ref())
        .map(|&id| (id, id - (input.lo % id)))
        .min_by_key(|&(_id, t)| t)
        .unwrap();
    id * t
}

fn solve2(ts: Vec<Option<i64>>) -> i64 {
    for (&t1, &t2) in ts.iter().filter_map(|t| t.as_ref()).tuple_combinations() {
        assert_eq!(gcd(t1 as i64, t2 as i64), 1);
    }
    // We need t such that:
    //   t === 0 (mod ts[0])
    //   t === -1 (mod ts[1])
    //   t === -2 (mod ts[2])
    let mut eqns: Vec<(i64, i64)> = ts
        .into_iter()
        .enumerate()
        .filter_map(|(idx, t)| Some((idx as i64, t? as i64)))
        .collect();
    eqns.sort_by_key(|(_idx, t)| -t);

    let mut x = 0;
    let mut dx = 1;
    for (idx, t) in eqns {
        // looking for x + k * dx === -i (mod t)
        let k = (0..).find(|k| (x + k * dx + idx) % t == 0).unwrap();
        x = x + k * dx;
        dx *= t;
    }
    x
}

fn gcd(m: i64, n: i64) -> i64 {
    if n == 0 {
        m
    } else {
        gcd(n, m % n)
    }
}
#[cfg(test)]
mod test {
    use super::{gcd, solve1, solve2, Schedule};

    const SMALL: &str = r"
        939
        7,13,x,x,59,x,31,19
    ";
    fn parse(input: &str) -> Schedule {
        let mut lines = input.trim().lines().map(|l| l.trim());
        let lo = lines.next().unwrap().parse().unwrap();
        let ts = lines
            .next()
            .unwrap()
            .split(',')
            .map(|t| t.parse().ok())
            .collect();
        Schedule { lo, ts }
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

    #[test]
    fn small2() {
        assert_eq!(solve2(parse(SMALL).ts), 1068781);
        assert_eq!(solve2(parse("0\n17,x,13,19").ts), 3417);
        assert_eq!(solve2(parse("0\n67,7,59,61").ts), 754018);
        assert_eq!(solve2(parse("0\n67,x,7,59,61").ts), 779210);
        assert_eq!(solve2(parse("0\n67,7,x,59,61").ts), 1261476);
        assert_eq!(solve2(parse("0\n1789,37,47,1889").ts), 1202161486);
    }

    #[test]
    fn gcd1() {
        assert_eq!(gcd(3, 9), 3);
        assert_eq!(gcd(4, 10), 2);
        assert_eq!(gcd(13, 19), 1);
    }

    #[test]
    fn normal2() {
        let raw = std::fs::read_to_string("data/day13.input").unwrap();
        let input = parse(&raw);
        assert_eq!(solve2(input.ts), 471793476184394);
    }
}
