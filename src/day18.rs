use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take_while1,
    character::complete::{multispace0, one_of},
    combinator::{all_consuming, map, map_res},
    multi::fold_many0,
    sequence::{delimited, pair, preceded},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Expr {
    Literal(i64),
    Add(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
}
impl Expr {
    fn add(a: Expr, b: Expr) -> Expr {
        Expr::Add(Box::new(a), Box::new(b))
    }
    fn mul(a: Expr, b: Expr) -> Expr {
        Expr::Multiply(Box::new(a), Box::new(b))
    }
}

impl FromStr for Expr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        all_consuming(expr_parser)(s)
            .map(|(_, parsed)| parsed)
            .map_err(|_| s.to_owned())
    }
}
fn expr_parser(input: &str) -> IResult<&str, Expr> {
    alt((binary_op_parser, literal_parser))(input)
}

fn binary_op_parser(input: &str) -> IResult<&str, Expr> {
    let (input, t0) = term_parser(input)?;
    fold_many0(
        pair(op_parser, term_parser),
        t0,
        |acc, (op, term)| match op {
            '+' => Expr::add(acc, term),
            '*' => Expr::mul(acc, term),
            _ => unreachable!(),
        },
    )(input)
}
fn op_parser(input: &str) -> IResult<&str, char> {
    preceded(multispace0, one_of("+*"))(input)
}
fn term_parser(input: &str) -> IResult<&str, Expr> {
    preceded(multispace0, alt((paren_parser, literal_parser)))(input)
}
fn paren_parser(input: &str) -> IResult<&str, Expr> {
    delimited(tag("("), expr_parser, tag(")"))(input)
}
fn literal_parser(input: &str) -> IResult<&str, Expr> {
    map(int_parser, |n| Expr::Literal(n))(input)
}
fn int_parser(input: &str) -> IResult<&str, i64> {
    map_res(take_while1(|c: char| c.is_digit(10)), |s: &str| s.parse())(input)
}

fn evaluate(expr: Expr) -> i64 {
    match expr {
        Expr::Literal(v) => v,
        Expr::Add(a, b) => evaluate(*a) + evaluate(*b),
        Expr::Multiply(a, b) => evaluate(*a) * evaluate(*b),
    }
}

fn solve1(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .map(|line| {
            let expr = line.trim().parse().unwrap();
            evaluate(expr)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::{solve1, Expr};
    #[test]
    fn parser() {
        assert_eq!("3".parse::<Expr>().unwrap(), Expr::Literal(3));
        assert_eq!(
            "3 + 4".parse::<Expr>().unwrap(),
            Expr::add(Expr::Literal(3), Expr::Literal(4))
        );
        assert_eq!(
            "3 * 4".parse::<Expr>().unwrap(),
            Expr::mul(Expr::Literal(3), Expr::Literal(4))
        );
        assert_eq!(
            "3 + 4 *5".parse::<Expr>().unwrap(),
            Expr::mul(
                Expr::add(Expr::Literal(3), Expr::Literal(4)),
                Expr::Literal(5)
            )
        );
        assert_eq!(
            "3 + (4*5)".parse::<Expr>().unwrap(),
            Expr::add(
                Expr::Literal(3),
                Expr::mul(Expr::Literal(4), Expr::Literal(5))
            )
        );
    }

    #[test]
    fn small1() {
        let raw = r"
            2 * 3 + (4 * 5)
            5 + (8 * 3 + 9 + 3 * 4 * 3)
            5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
            ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
        ";
        assert_eq!(solve1(raw), 26335);
    }

    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day18.input").unwrap();
        assert_eq!(solve1(&raw), 42);
    }
}
