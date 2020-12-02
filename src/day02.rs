use lazy_static::lazy_static;
use regex::Regex;

use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct Policy {
    target: char,
    lo: usize,
    hi: usize,
}
struct Input {
    policy: Policy,
    password: String,
}
impl Input {
    fn is_valid(&self) -> bool {
        (self.policy.lo..=self.policy.hi).contains(
            &self
                .password
                .chars()
                .filter(|&c| c == self.policy.target)
                .count(),
        )
    }

    fn is_valid_2(&self) -> bool {
        let a = self.password.chars().nth(self.policy.lo - 1).unwrap();
        let b = self.password.chars().nth(self.policy.hi - 1).unwrap();
        (a == self.policy.target) ^ (b == self.policy.target)
    }
}

lazy_static! {
    static ref PATTERN: Regex =
        Regex::new(r"(?P<lo>\d+)-(?P<hi>\d+) (?P<target>\w): (?P<password>\w+)").unwrap();
}
impl FromStr for Input {
    type Err = String;
    fn from_str(s: &str) -> Result<Input, Self::Err> {
        let m = PATTERN.captures(s).ok_or(s.to_owned())?;
        Ok(Input {
            policy: Policy {
                target: m.name("target").unwrap().as_str().chars().next().unwrap(),
                lo: m.name("lo").unwrap().as_str().parse().unwrap(),
                hi: m.name("hi").unwrap().as_str().parse().unwrap(),
            },
            password: m.name("password").unwrap().as_str().to_owned(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::{Input, Policy};

    #[test]
    fn parser() {
        let Input { policy, password } = "1-3 a: abcde".parse().unwrap();
        assert_eq!(
            policy,
            Policy {
                target: 'a',
                lo: 1,
                hi: 3,
            }
        );
        assert_eq!(password, "abcde");
    }

    #[test]
    fn small1() {
        let raw = r"
            1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc
        ";
        let count = raw
            .trim()
            .lines()
            .map(|line| line.parse::<Input>().unwrap())
            .filter(|input| input.is_valid())
            .count();
        assert_eq!(count, 2);
    }

    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day02.input").unwrap();
        let count = raw
            .trim()
            .lines()
            .map(|line| line.parse::<Input>().unwrap())
            .filter(|input| input.is_valid())
            .count();
        assert_eq!(count, 467);
    }

    #[test]
    fn small2() {
        let raw = r"
            1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc
        ";
        let count = raw
            .trim()
            .lines()
            .map(|line| line.parse::<Input>().unwrap())
            .filter(|input| input.is_valid_2())
            .count();
        assert_eq!(count, 1);
    }

    #[test]
    fn normal2() {
        let raw = std::fs::read_to_string("data/day02.input").unwrap();
        let count = raw
            .trim()
            .lines()
            .map(|line| line.parse::<Input>().unwrap())
            .filter(|input| input.is_valid_2())
            .count();
        assert_eq!(count, 441);
    }
}
