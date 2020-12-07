use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

type Bag = String;
#[derive(Debug, Eq, PartialEq)]
struct Relation {
    bag: Bag,
    contents: Vec<(usize, Bag)>,
}
lazy_static! {
    static ref TOP_LEVEL: Regex = Regex::new(r"^(.+?) bags contain (.+).$").unwrap();
    static ref CONTENT: Regex = Regex::new(r"^([[:digit:]]+) (.+) bags?$").unwrap();
}
impl FromStr for Relation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let top = TOP_LEVEL
            .captures(s.trim())
            .ok_or(format!("invalid format: {}", s))?;
        let bag: Bag = top.get(1).unwrap().as_str().to_owned();
        let right = top.get(2).unwrap().as_str();
        let mut contents = vec![];
        if right != "no other bags" {
            for s1 in right.split(", ") {
                let content = CONTENT
                    .captures(s1)
                    .ok_or(format!("invalid content: {}", s1))?;
                let count = {
                    let count = content.get(1).unwrap().as_str();
                    count
                        .parse::<usize>()
                        .map_err(|_| format!(r"invalid count: {}", count))?
                };
                let bag = content.get(2).unwrap().as_str().to_owned();
                contents.push((count, bag));
            }
        }
        Ok(Relation { bag, contents })
    }
}

fn find_containers(relations: &[Relation], target: Bag) -> HashSet<Bag> {
    let mut parents: HashMap<Bag, Vec<Bag>> = HashMap::new();
    for r in relations {
        for (_, child) in &r.contents {
            parents
                .entry(child.clone())
                .or_default()
                .push(r.bag.clone());
        }
    }
    let mut containers = HashSet::new();
    let mut frontier: Vec<Bag> = vec![target];
    while let Some(bag) = frontier.pop() {
        if let Some(ps) = parents.get(&bag) {
            for p in ps {
                containers.insert(p.clone());
                frontier.push(p.clone());
            }
        }
    }
    containers
}

fn count_contents(relations: &[Relation], target: Bag) -> usize {
    let contents = relations
        .iter()
        .find(|r| r.bag == target)
        .map(|r| r.contents.clone())
        .unwrap_or_default();
    contents
        .into_iter()
        .map(|(k, b)| k * (1 + count_contents(relations, b)))
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::{count_contents, find_containers, Relation};

    const SMALL: &str = r"
        light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.
    ";

    #[test]
    fn parser() {
        let lines: Vec<&str> = SMALL.trim().lines().collect();
        assert_eq!(
            lines[0].parse::<Relation>().unwrap(),
            Relation {
                bag: "light red".to_owned(),
                contents: vec![
                    (1, "bright white".to_owned()),
                    (2, "muted yellow".to_owned())
                ]
            }
        );
        assert_eq!(
            lines[2].parse::<Relation>().unwrap(),
            Relation {
                bag: "bright white".to_owned(),
                contents: vec![(1, "shiny gold".to_owned()),]
            }
        );
        assert_eq!(
            lines[8].parse::<Relation>().unwrap(),
            Relation {
                bag: "dotted black".to_owned(),
                contents: vec![]
            }
        );
    }

    #[test]
    fn small1() {
        let relations: Vec<Relation> = SMALL
            .trim()
            .lines()
            .map(|l| l.parse::<Relation>().unwrap())
            .collect();
        assert_eq!(
            find_containers(&relations, "shiny gold".to_owned()).len(),
            4
        );
    }

    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day07.input").unwrap();
        let relations: Vec<Relation> = raw
            .trim()
            .lines()
            .map(|l| l.parse::<Relation>().unwrap())
            .collect();
        assert_eq!(
            find_containers(&relations, "shiny gold".to_owned()).len(),
            197
        );
    }

    #[test]
    fn small2() {
        let relations: Vec<Relation> = SMALL
            .trim()
            .lines()
            .map(|l| l.parse::<Relation>().unwrap())
            .collect();
        assert_eq!(count_contents(&relations, "shiny gold".to_owned()), 32);
    }

    #[test]
    fn normal2() {
        let raw = std::fs::read_to_string("data/day07.input").unwrap();
        let relations: Vec<Relation> = raw
            .trim()
            .lines()
            .map(|l| l.parse::<Relation>().unwrap())
            .collect();
        assert_eq!(count_contents(&relations, "shiny gold".to_owned()), 85324);
    }
}
