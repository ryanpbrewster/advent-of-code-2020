use std::collections::HashMap;

enum Op {
    SetMask(Vec<u8>),
    Write { addr: u64, value: u64 },
}

const BITS: usize = 36;
fn solve1(ops: Vec<Op>) -> HashMap<u64, u64> {
    let mut registers = HashMap::new();
    let mut mask = vec![0; BITS];
    for op in ops {
        match op {
            Op::SetMask(m) => {
                mask = m;
            }
            Op::Write { addr, value } => {
                registers.insert(addr, mask_value(&mask, value));
            }
        };
    }
    registers
}

fn mask_value(mask: &[u8], value: u64) -> u64 {
    (0..BITS).fold(0, |acc, i| {
        let v = match mask[i] {
            b'0' => 0,
            b'1' => 1,
            b'X' => (value >> BITS - i - 1) & 1,
            other => panic!("illegal mask entry: {}", other),
        };
        2 * acc + v
    })
}

fn solve2(ops: Vec<Op>) -> HashMap<u64, u64> {
    let mut registers = HashMap::new();
    let mut mask = vec![0; BITS];
    for op in ops {
        match op {
            Op::SetMask(m) => {
                mask = m;
            }
            Op::Write { addr, value } => {
                for a in mask_addr(&mask, addr) {
                    registers.insert(a, value);
                }
            }
        };
    }
    registers
}

fn mask_addr(mask: &[u8], addr: u64) -> Vec<u64> {
    let mut addrs = vec![0];
    for i in 0..BITS {
        let vs = match mask[i] {
            b'0' => vec![(addr >> BITS - i - 1) & 1],
            b'1' => vec![1],
            b'X' => vec![0, 1],
            other => panic!("illegal mask entry: {}", other),
        };
        let mut next = Vec::with_capacity(addrs.len() * vs.len());
        for &a in &addrs {
            for &v in &vs {
                next.push(2 * a + v);
            }
        }
        std::mem::swap(&mut addrs, &mut next);
    }
    addrs
}

#[cfg(test)]
mod test {
    use super::{solve1, solve2, Op};
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref MASK: Regex = Regex::new(r"^mask = ([01X]{36})$").unwrap();
        static ref WRITE: Regex = Regex::new(r"^mem\[([[:digit:]]+)\] = ([[:digit:]]+)$").unwrap();
    }
    fn parse(input: &str) -> Vec<Op> {
        input
            .trim()
            .lines()
            .map(|l| {
                let s = l.trim();
                if let Some(m) = MASK.captures(s) {
                    let mask = m.get(1).unwrap().as_str();
                    Op::SetMask(mask.to_owned().into_bytes())
                } else if let Some(m) = WRITE.captures(s) {
                    let addr = m.get(1).unwrap().as_str().parse().unwrap();
                    let value = m.get(2).unwrap().as_str().parse().unwrap();
                    Op::Write { addr, value }
                } else {
                    panic!("invalid line: {}", s);
                }
            })
            .collect()
    }
    #[test]
    fn small1() {
        let raw = r"
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            mem[8] = 11
            mem[7] = 101
            mem[8] = 0
        ";
        let input = parse(raw);
        assert_eq!(solve1(input).values().sum::<u64>(), 165);
    }

    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day14.input").unwrap();
        let input = parse(&raw);
        assert_eq!(solve1(input).values().sum::<u64>(), 16003257187056);
    }

    #[test]
    fn small2() {
        let raw = r"
            mask = 000000000000000000000000000000X1001X
            mem[42] = 100
            mask = 00000000000000000000000000000000X0XX
            mem[26] = 1
        ";
        let input = parse(raw);
        assert_eq!(solve2(input).values().sum::<u64>(), 208);
    }

    #[test]
    fn normal2() {
        let raw = std::fs::read_to_string("data/day14.input").unwrap();
        let input = parse(&raw);
        assert_eq!(solve2(input).values().sum::<u64>(), 3219837697833);
    }
}
