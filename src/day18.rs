use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take_while1,
    character::complete::{multispace0, one_of},
    combinator::{map, map_res},
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

fn lr_parser(input: &str) -> IResult<&str, Expr> {
    let (input, t0) = lr_term_parser(input)?;
    fold_many0(
        pair(one_of("+*"), lr_term_parser),
        t0,
        |acc, (op, term)| match op {
            '+' => Expr::add(acc, term),
            '*' => Expr::mul(acc, term),
            _ => unreachable!(),
        },
    )(input)
}
fn lr_term_parser(input: &str) -> IResult<&str, Expr> {
    delimited(
        multispace0,
        alt((lr_paren_parser, literal_parser)),
        multispace0,
    )(input)
}
fn lr_paren_parser(input: &str) -> IResult<&str, Expr> {
    delimited(tag("("), lr_parser, tag(")"))(input)
}

fn mul_parser(input: &str) -> IResult<&str, Expr> {
    let (input, t0) = add_parser(input)?;
    fold_many0(preceded(tag("*"), add_parser), t0, |acc, term| {
        Expr::mul(acc, term)
    })(input)
}
fn add_parser(input: &str) -> IResult<&str, Expr> {
    let (input, t0) = mul_term_parser(input)?;
    fold_many0(preceded(tag("+"), mul_term_parser), t0, |acc, term| {
        Expr::add(acc, term)
    })(input)
}
fn mul_term_parser(input: &str) -> IResult<&str, Expr> {
    delimited(
        multispace0,
        alt((mul_paren_parser, literal_parser)),
        multispace0,
    )(input)
}
fn mul_paren_parser(input: &str) -> IResult<&str, Expr> {
    delimited(tag("("), mul_parser, tag(")"))(input)
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
            let expr = lr_parser(line.trim()).unwrap().1;
            evaluate(expr)
        })
        .sum()
}

fn solve2(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .map(|line| {
            let expr = mul_parser(line.trim()).unwrap().1;
            evaluate(expr)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::{lr_parser, solve1, solve2, Expr};
    #[test]
    fn parser() {
        assert_eq!(lr_parser("3").unwrap().1, Expr::Literal(3));
        assert_eq!(
            lr_parser("3 + 4").unwrap().1,
            Expr::add(Expr::Literal(3), Expr::Literal(4))
        );
        assert_eq!(
            lr_parser("3 * 4").unwrap().1,
            Expr::mul(Expr::Literal(3), Expr::Literal(4))
        );
        assert_eq!(
            lr_parser("3 + 4 *5").unwrap().1,
            Expr::mul(
                Expr::add(Expr::Literal(3), Expr::Literal(4)),
                Expr::Literal(5)
            )
        );
        assert_eq!(
            lr_parser("3 + (4*5)").unwrap().1,
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
        assert_eq!(solve1(&raw), 25190263477788);
    }

    #[test]
    fn normal2() {
        let raw = std::fs::read_to_string("data/day18.input").unwrap();
        assert_eq!(solve2(&raw), 297139939002972);
    }
}
