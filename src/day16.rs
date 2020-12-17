use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

use nom::{
    bytes::complete::tag,
    bytes::complete::take_while1,
    character::complete::{multispace0, multispace1},
    combinator::{all_consuming, map_res},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct FieldSpec {
    name: String,
    ranges: Vec<RangeInclusive<usize>>,
}

type Ticket = Vec<usize>;
struct Input {
    specs: HashMap<String, FieldSpec>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, input) = all_consuming(delimited(multispace0, input_parser, multispace0))(s)
            .map_err(|_| s.to_owned())?;
        Ok(input)
    }
}
fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, fields) = separated_list1(multispace1, field_parser)(input)?;
    let (input, _) = delimited(multispace0, tag("your ticket:"), multispace0)(input)?;
    let (input, my_ticket) = ticket_parser(input)?;
    let (input, _) = delimited(multispace0, tag("nearby tickets:"), multispace0)(input)?;
    let (input, nearby_tickets) = separated_list1(multispace1, ticket_parser)(input)?;
    let parsed = Input {
        specs: fields
            .into_iter()
            .map(|field| (field.name.clone(), field))
            .collect(),
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
    let is_valid = |v| {
        input
            .specs
            .iter()
            .any(|(_, spec)| spec.ranges.iter().any(|range| range.contains(v)))
    };
    input
        .nearby_tickets
        .iter()
        .flat_map(|ticket| ticket.iter())
        .filter(|v| !is_valid(v))
        .sum()
}

#[cfg(test)]
mod test {
    use super::{field_parser, range_parser, solve1, FieldSpec, Input};

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
}
