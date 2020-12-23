use std::collections::{hash_map::DefaultHasher, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

use nom::{
    bytes::complete::{tag, take_while},
    character::complete::multispace1,
    combinator::{map, map_res},
    multi::separated_list1,
    IResult,
};

type Player = VecDeque<usize>;
fn input_parser(input: &str) -> IResult<&str, (Vec<usize>, Vec<usize>)> {
    let (input, _) = tag("Player 1:")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, p1) = player_parser(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("Player 2:")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, p2) = player_parser(input)?;
    Ok((input, (p1, p2)))
}
fn player_parser(input: &str) -> IResult<&str, Vec<usize>> {
    map(separated_list1(multispace1, usize_parser), |cards| {
        cards.into_iter().collect()
    })(input)
}
fn usize_parser(input: &str) -> IResult<&str, usize> {
    map_res(take_while(|c: char| c.is_digit(10)), |s: &str| s.parse())(input)
}

fn solve1(p1: &[usize], p2: &[usize]) -> usize {
    let mut p1: VecDeque<usize> = p1.iter().copied().collect();
    let mut p2: VecDeque<usize> = p2.iter().copied().collect();

    while !p1.is_empty() && !p2.is_empty() {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }
    if p1.is_empty() {
        score(&p2)
    } else {
        score(&p1)
    }
}

fn solve2(p1: &[usize], p2: &[usize]) -> usize {
    let mut p1: VecDeque<usize> = p1.iter().copied().collect();
    let mut p2: VecDeque<usize> = p2.iter().copied().collect();
    let winner = play_game(&mut p1, &mut p2, 0);
    match winner {
        Winner::P1 => score(&p1),
        Winner::P2 => score(&p2),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Winner {
    P1,
    P2,
}

fn snapshot(p1: &VecDeque<usize>, p2: &VecDeque<usize>) -> u64 {
    let mut h = DefaultHasher::new();
    (p1, p2).hash(&mut h);
    h.finish()
}
fn play_game(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>, depth: usize) -> Winner {
    let mut seen: HashSet<u64> = HashSet::new();
    loop {
        if p1.is_empty() {
            return Winner::P2;
        }
        if p2.is_empty() {
            return Winner::P1;
        }
        if !seen.insert(snapshot(&p1, &p2)) {
            return Winner::P1;
        }
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        let winner = if p1.len() >= c1 && p2.len() >= c2 {
            let mut p1: VecDeque<usize> = p1.iter().copied().take(c1).collect();
            let mut p2: VecDeque<usize> = p2.iter().copied().take(c2).collect();
            play_game(&mut p1, &mut p2, depth + 1)
        } else if c1 > c2 {
            Winner::P1
        } else {
            Winner::P2
        };
        match winner {
            Winner::P1 => {
                p1.push_back(c1);
                p1.push_back(c2);
            }
            Winner::P2 => {
                p2.push_back(c2);
                p2.push_back(c1);
            }
        }
    }
}

fn score(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(idx, v)| (idx + 1) * v)
        .sum()
}

#[cfg(test)]
mod test {
    use super::{input_parser, player_parser, solve1, solve2};
    #[test]
    fn parser_test() {
        let p = player_parser("3 1 4").unwrap().1;
        assert_eq!(p, vec![3, 1, 4]);
        let (p1, p2) = input_parser(SMALL.trim()).unwrap().1;
        assert_eq!(p1, vec![9, 2, 6, 3, 1]);
        assert_eq!(p2, vec![5, 8, 4, 7, 10]);
    }

    const SMALL: &str = r"
        Player 1:
        9
        2
        6
        3
        1

        Player 2:
        5
        8
        4
        7
        10
    ";
    #[test]
    fn small1() {
        let (p1, p2) = input_parser(SMALL.trim()).unwrap().1;
        assert_eq!(solve1(&p1, &p2), 306);
    }
    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day22.input").unwrap();
        let (p1, p2) = input_parser(raw.trim()).unwrap().1;
        assert_eq!(solve1(&p1, &p2), 31754);
    }

    #[test]
    fn small2() {
        let (p1, p2) = input_parser(SMALL.trim()).unwrap().1;
        assert_eq!(solve2(&p1, &p2), 291);
    }

    #[test]
    fn normal2() {
        let raw = std::fs::read_to_string("data/day22.input").unwrap();
        let (p1, p2) = input_parser(raw.trim()).unwrap().1;
        assert_eq!(solve2(&p1, &p2), 35436);
    }
}
