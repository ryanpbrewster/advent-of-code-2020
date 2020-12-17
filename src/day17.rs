use std::collections::HashSet;
use std::hash::Hash;

trait Cell: Eq + PartialEq + Sized + Hash {
    fn of(i: i32, j: i32) -> Self;
    fn neighbors(&self) -> Vec<Self>;
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct P3(i32, i32, i32);
impl Cell for P3 {
    fn of(i: i32, j: i32) -> P3 {
        P3(i, j, 0)
    }
    fn neighbors(&self) -> Vec<P3> {
        let mut ns = Vec::with_capacity(27);
        let P3(i, j, k) = self;
        for di in -1..=1 {
            for dj in -1..=1 {
                for dk in -1..=1 {
                    ns.push(P3(i + di, j + dj, k + dk))
                }
            }
        }
        ns
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct P4(i32, i32, i32, i32);
impl Cell for P4 {
    fn of(i: i32, j: i32) -> P4 {
        P4(i, j, 0, 0)
    }
    fn neighbors(&self) -> Vec<P4> {
        let mut ns = Vec::with_capacity(81);
        let P4(i, j, k, l) = self;
        for di in -1..=1 {
            for dj in -1..=1 {
                for dk in -1..=1 {
                    for dl in -1..=1 {
                        ns.push(P4(i + di, j + dj, k + dk, l + dl))
                    }
                }
            }
        }
        ns
    }
}

struct Universe<C: Cell> {
    active: HashSet<C>,
}
impl<C: Cell> Universe<C> {
    fn count_active(&self) -> usize {
        self.active.len()
    }
    fn step(&mut self) {
        let all: HashSet<C> = self
            .active
            .iter()
            .flat_map(|p| p.neighbors().into_iter())
            .collect();
        let mut next = HashSet::new();
        for p in all {
            let adj = p
                .neighbors()
                .into_iter()
                .filter(|n| *n != p && self.active.contains(n))
                .count();
            if self.active.contains(&p) && (adj == 2 || adj == 3) {
                next.insert(p);
            } else if !self.active.contains(&p) && adj == 3 {
                next.insert(p);
            }
        }
        std::mem::swap(&mut self.active, &mut next);
    }
}

fn parse_grid<C: Cell>(raw: &str) -> Universe<C> {
    let mut active = HashSet::new();
    for (i, line) in raw.trim().lines().enumerate() {
        for (j, ch) in line.trim().chars().enumerate() {
            if ch == '#' {
                active.insert(C::of(i as i32, j as i32));
            }
        }
    }
    Universe { active }
}

fn solve1(mut init: Universe<P3>) -> usize {
    for _ in 0..6 {
        init.step();
    }
    init.count_active()
}

fn solve2(mut init: Universe<P4>) -> usize {
    for _ in 0..6 {
        init.step();
    }
    init.count_active()
}

#[cfg(test)]
mod test {
    use super::{parse_grid, solve1, solve2, P3};

    const SMALL: &str = r"
        .#.
        ..#
        ###
    ";

    #[test]
    fn parser() {
        assert_eq!(parse_grid::<P3>(SMALL).count_active(), 5);
    }

    #[test]
    fn small1() {
        assert_eq!(solve1(parse_grid(SMALL)), 112);
    }

    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day17.input").unwrap();
        assert_eq!(solve1(parse_grid(&raw)), 353);
    }

    #[test]
    fn small2() {
        assert_eq!(solve2(parse_grid(SMALL)), 848);
    }

    #[test]
    fn normal2() {
        let raw = std::fs::read_to_string("data/day17.input").unwrap();
        assert_eq!(solve2(parse_grid(&raw)), 2472);
    }
}
