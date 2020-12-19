use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take_while1,
    character::complete::{anychar, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Rule {
    Literal(char),
    Subrule(Vec<usize>),
    Alt(Vec<usize>, Vec<usize>),
}

struct Grammar {
    rules: HashMap<usize, Rule>,
}
impl Grammar {
    fn accepts<'a>(&self, input: &'a str) -> bool {
        self.check(vec![0], input)
    }
    fn check<'a>(&self, mut rules: Vec<usize>, input: &str) -> bool {
        if rules.is_empty() && input.is_empty() {
            return true;
        }
        let first_rule = match rules.pop() {
            Some(id) => &self.rules[&id],
            None => return false,
        };
        match first_rule {
            Rule::Literal(c) => input.starts_with(*c) && self.check(rules, &input[1..]),
            Rule::Subrule(sub) => {
                rules.extend(sub.iter().rev());
                self.check(rules, input)
            }
            Rule::Alt(a, b) => {
                let mut ra = rules.clone();
                ra.extend(a.iter().rev());

                let mut rb = rules;
                rb.extend(b.iter().rev());
                self.check(ra, input) || self.check(rb, input)
            }
        }
    }
}

fn rule_parser(input: &str) -> IResult<&str, (usize, Rule)> {
    separated_pair(
        usize_parser,
        tag(": "),
        alt((literal_parser, subrule_parser)),
    )(input)
}
fn subrule_parser(input: &str) -> IResult<&str, Rule> {
    map(
        separated_list1(tag(" | "), separated_list1(space1, usize_parser)),
        |vs| match vs.len() {
            1 => Rule::Subrule(vs[0].clone()),
            2 => Rule::Alt(vs[0].clone(), vs[1].clone()),
            _ => panic!(),
        },
    )(input)
}
fn literal_parser(input: &str) -> IResult<&str, Rule> {
    map(delimited(tag("\""), u8_parser, tag("\"")), |n| {
        Rule::Literal(n)
    })(input)
}
fn u8_parser(input: &str) -> IResult<&str, char> {
    anychar(input)
}
fn usize_parser(input: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_digit(10)), |s: &str| s.parse())(input)
}

fn parse_grammar(input: &str) -> (Grammar, Vec<&str>) {
    let mut lines = input.trim().lines().map(|l| l.trim());
    let mut rules = HashMap::new();
    while let Some(l) = lines.next() {
        if l.is_empty() {
            break;
        }
        let (id, rule) = rule_parser(l).unwrap().1;
        rules.insert(id, rule);
    }
    let grammar = Grammar { rules };
    let msgs = lines.collect();
    (grammar, msgs)
}
fn solve1(input: &str) -> usize {
    let (grammar, msgs) = parse_grammar(input);
    msgs.into_iter().filter(|l| grammar.accepts(l)).count()
}
fn solve2(input: &str) -> usize {
    let (mut grammar, msgs) = parse_grammar(input);
    grammar.rules.insert(8, Rule::Alt(vec![42], vec![42, 8]));
    grammar
        .rules
        .insert(11, Rule::Alt(vec![42, 31], vec![42, 11, 31]));

    msgs.into_iter().filter(|l| grammar.accepts(l)).count()
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::{rule_parser, solve1, solve2, Grammar, Rule};
    #[test]
    fn parser_literal() {
        let (id, rule) = rule_parser(r#" 0: "a" "#.trim()).unwrap().1;
        assert_eq!(id, 0);
        assert_eq!(rule, Rule::Literal('a'));
    }
    #[test]
    fn parser_subrule() {
        let (id, rule) = rule_parser(r#" 1: 2 3 | 5 6 7 "#.trim()).unwrap().1;
        assert_eq!(id, 1);
        assert_eq!(rule, Rule::Alt(vec![2, 3], vec![5, 6, 7]));
    }

    #[test]
    fn small1() {
        let input = r#"
            0: 4 1 5
            1: 2 3 | 3 2
            2: 4 4 | 5 5
            3: 4 5 | 5 4
            4: "a"
            5: "b"

            ababbb
            bababa
            abbbab
            aaabbb
            aaaabbb
        "#;
        assert_eq!(solve1(input), 2);
    }

    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day19.input").unwrap();
        assert_eq!(solve1(&raw), 147);
    }

    #[test]
    fn tiny2() {
        let mut rules = HashMap::new();
        rules.insert(0, Rule::Alt(vec![1, 0], vec![1]));
        rules.insert(1, Rule::Literal('a'));
        let g = Grammar { rules };
        assert!(g.accepts("a"));
        assert!(g.accepts("aa"));
        assert!(g.accepts("aaaaaaaaaa"));
    }

    #[test]
    fn small2() {
        let input = r#"
            42: 9 14 | 10 1
            9: 14 27 | 1 26
            10: 23 14 | 28 1
            1: "a"
            11: 42 31
            5: 1 14 | 15 1
            19: 14 1 | 14 14
            12: 24 14 | 19 1
            16: 15 1 | 14 14
            31: 14 17 | 1 13
            6: 14 14 | 1 14
            2: 1 24 | 14 4
            0: 8 11
            13: 14 3 | 1 12
            15: 1 | 14
            17: 14 2 | 1 7
            23: 25 1 | 22 14
            28: 16 1
            4: 1 1
            20: 14 14 | 1 15
            3: 5 14 | 16 1
            27: 1 6 | 14 18
            14: "b"
            21: 14 1 | 1 14
            25: 1 1 | 1 14
            22: 14 14
            8: 42
            26: 14 22 | 1 20
            18: 15 15
            7: 14 5 | 1 21
            24: 14 1
            
            abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
            bbabbbbaabaabba
            babbbbaabbbbbabbbbbbaabaaabaaa
            aaabbbbbbaaaabaababaabababbabaaabbababababaaa
            bbbbbbbaaaabbbbaaabbabaaa
            bbbababbbbaaaaaaaabbababaaababaabab
            ababaaaaaabaaab
            ababaaaaabbbaba
            baabbaaaabbaaaababbaababb
            abbbbabbbbaaaababbbbbbaaaababb
            aaaaabbaabaaaaababaa
            aaaabbaaaabbaaa
            aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
            babaaabbbaaabaababbaabababaaab
            aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
        "#;
        assert_eq!(solve1(input), 3);
        assert_eq!(solve2(input), 12);
    }

    #[test]
    fn normal2() {
        let raw = std::fs::read_to_string("data/day19.input").unwrap();
        assert_eq!(solve2(&raw), 263);
    }
}
