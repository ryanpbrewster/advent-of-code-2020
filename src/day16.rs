use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
    str::FromStr,
};

use nom::{
    bytes::complete::tag,
    bytes::complete::take_while1,
    character::complete::{multispace0, multispace1},
    combinator::{all_consuming, map_res},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
struct FieldSpec {
    name: String,
    ranges: Vec<RangeInclusive<usize>>,
}
impl FieldSpec {
    fn allows(&self, v: usize) -> bool {
        self.ranges.iter().any(|r| r.contains(&v))
    }
}

type Ticket = Vec<usize>;
struct Input {
    specs: Vec<FieldSpec>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        all_consuming(input_parser)(s)
            .map(|(_, parsed)| parsed)
            .map_err(|_| s.to_owned())
    }
}
fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, specs) = separated_list1(multispace1, field_parser)(input)?;
    let (input, _) = delimited(multispace0, tag("your ticket:"), multispace0)(input)?;
    let (input, my_ticket) = ticket_parser(input)?;
    let (input, _) = delimited(multispace0, tag("nearby tickets:"), multispace0)(input)?;
    let (input, nearby_tickets) = separated_list1(multispace1, ticket_parser)(input)?;
    let parsed = Input {
        specs,
        my_ticket,
        nearby_tickets,
    };
    Ok((input, parsed))
}

fn ticket_parser(input: &str) -> IResult<&str, Ticket> {
    separated_list1(tag(","), usize_parser)(input)
}
fn field_parser(input: &str) -> IResult<&str, FieldSpec> {
    let (input, name) = take_while1(|c| c != ':')(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, ranges) = separated_list1(tag(" or "), range_parser)(input)?;
    Ok((
        input,
        FieldSpec {
            name: name.to_owned(),
            ranges,
        },
    ))
}
fn range_parser(input: &str) -> IResult<&str, RangeInclusive<usize>> {
    let (input, lo) = usize_parser(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, hi) = usize_parser(input)?;
    Ok((input, lo..=hi))
}
fn usize_parser(input: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_digit(10)), |s: &str| s.parse())(input)
}

fn solve1(input: &Input) -> usize {
    let is_valid = |v| input.specs.iter().any(|spec| spec.allows(v));
    input
        .nearby_tickets
        .iter()
        .filter_map(|ticket| ticket.iter().find(|&&v| !is_valid(v)))
        .sum()
}

type LabeledTicket = Vec<String>;
fn solve2(input: &Input) -> LabeledTicket {
    let is_valid = |v| input.specs.iter().any(|spec| spec.allows(v));
    let valid_tickets: Vec<Ticket> = input
        .nearby_tickets
        .iter()
        .filter(|t| t.iter().all(|&v| is_valid(v)))
        .cloned()
        .collect();
    let ticket_size = input.specs.len();
    struct Possibility {
        field: FieldSpec,
        candidates: HashSet<usize>,
    }
    let mut possibilities: Vec<Possibility> = input
        .specs
        .iter()
        .map(|field| Possibility {
            field: field.clone(),
            candidates: (0..ticket_size).collect(),
        })
        .collect();
    for ticket in valid_tickets {
        for (i, &v) in ticket.iter().enumerate() {
            for Possibility { field, candidates } in possibilities.iter_mut() {
                if !field.allows(v) {
                    candidates.remove(&i);
                }
            }
        }
    }
    let mut mapping: HashMap<usize, String> = HashMap::new();
    while mapping.len() < ticket_size {
        // Find a name with an obvious mapping.
        let trivial: &Possibility = possibilities
            .iter()
            .find(|p| p.candidates.len() == 1)
            .unwrap();
        let idx: usize = *trivial.candidates.iter().next().unwrap();
        mapping.insert(idx, trivial.field.name.clone());
        // No other field is allowed to use this index now.
        for p in possibilities.iter_mut() {
            p.candidates.remove(&idx);
        }
    }
    (0..ticket_size).map(|idx| mapping[&idx].clone()).collect()
}

#[cfg(test)]
mod test {
    use super::{field_parser, range_parser, solve1, solve2, FieldSpec, Input};

    const SMALL: &str = r"
        class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50

        your ticket:
        7,1,14

        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12
    ";

    #[test]
    fn parser() {
        assert_eq!(range_parser("3-7").unwrap().1, 3..=7);
        assert_eq!(
            field_parser("hello: 3-7 or 9-12").unwrap().1,
            FieldSpec {
                name: "hello".to_owned(),
                ranges: vec![3..=7, 9..=12]
            }
        );
    }

    #[test]
    fn small1() {
        let input = SMALL.trim().parse::<Input>().unwrap();
        assert_eq!(solve1(&input), 71);
    }

    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day16.input").unwrap();
        let input = raw.trim().parse::<Input>().unwrap();
        assert_eq!(solve1(&input), 18142);
    }

    #[test]
    fn small2() {
        let input = SMALL.trim().parse::<Input>().unwrap();
        assert_eq!(solve2(&input), vec!["row", "class", "seat"]);
    }

    #[test]
    fn normal2() {
        let raw = std::fs::read_to_string("data/day16.input").unwrap();
        let input = raw.trim().parse::<Input>().unwrap();
        let labels = solve2(&input);
        let computed = input
            .my_ticket
            .into_iter()
            .zip(labels.into_iter())
            .filter(|(_, name)| name.starts_with("departure"))
            .map(|(v, _)| v)
            .product::<usize>();
        assert_eq!(computed, 1069784384303);
    }
}
