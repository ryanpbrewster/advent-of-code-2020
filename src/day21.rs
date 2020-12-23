use std::collections::{hash_map::Entry, HashMap, HashSet};

use nom::character::is_alphabetic;
use nom::{
    branch::alt,
    bytes::{
        complete::{tag, take_while},
        streaming::take_while1,
    },
    character::complete::{multispace0, multispace1, space1},
    combinator::{all_consuming, map, map_res, value},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

fn input_parser(input: &str) -> IResult<&str, Vec<Food>> {
    separated_list1(multispace1, food_parser)(input)
}
fn food_parser(input: &str) -> IResult<&str, Food> {
    let (input, ingredients) = separated_list1(space1, token_parser)(input)?;
    let (input, _) = space1(input)?;
    let (input, allergens) = delimited(
        tag("(contains "),
        separated_list1(tag(", "), token_parser),
        tag(")"),
    )(input)?;
    Ok((
        input,
        Food {
            ingredients: ingredients.into_iter().collect(),
            allergens: allergens.into_iter().collect(),
        },
    ))
}
fn token_parser(input: &str) -> IResult<&str, String> {
    let (input, token): (&str, &str) = take_while1(|c: char| c.is_alphabetic())(input)?;
    Ok((input, token.to_owned()))
}

fn solve1(foods: &[Food]) -> usize {
    type Ingredient = String;
    type Allergen = String;
    let mut candidates: HashMap<Allergen, HashSet<Ingredient>> = HashMap::new();
    for food in foods {
        for allergen in &food.allergens {
            match candidates.entry(allergen.clone()) {
                Entry::Occupied(mut cur) => {
                    cur.get_mut()
                        .retain(|ingredient| food.ingredients.contains(ingredient));
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(food.ingredients.iter().cloned().collect());
                }
            };
        }
    }
    let all: HashSet<Ingredient> = foods
        .iter()
        .flat_map(|food| food.ingredients.iter())
        .cloned()
        .collect();
    let suspicious: HashSet<Ingredient> = candidates
        .values()
        .flat_map(|ps| ps.iter())
        .cloned()
        .collect();
    let innocent: HashSet<Ingredient> = all.difference(&suspicious).cloned().collect();
    let occurrences = foods
        .iter()
        .flat_map(|food| food.ingredients.iter())
        .filter(|&ingredient| innocent.contains(ingredient))
        .count();
    occurrences
}

#[cfg(test)]
mod test {
    use super::{food_parser, input_parser, solve1, token_parser, Food};

    #[test]
    fn parser_test() {
        assert_eq!(token_parser("hello world").unwrap().1, "hello".to_owned());
        assert_eq!(
            food_parser("abc def (contains pqr, tuv)").unwrap().1,
            Food {
                ingredients: vec!["abc", "def"].iter().map(|&s| s.to_owned()).collect(),
                allergens: vec!["pqr", "tuv"].iter().map(|&s| s.to_owned()).collect(),
            }
        );
    }

    const SMALL: &str = r"
        mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
        trh fvjkl sbzzf mxmxvkd (contains dairy)
        sqjhc fvjkl (contains soy)
        sqjhc mxmxvkd sbzzf (contains fish)
    ";
    #[test]
    fn small1() {
        let parsed = input_parser(SMALL.trim()).unwrap().1;
        assert_eq!(solve1(&parsed), 5);
    }
    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day21.input").unwrap();
        let parsed = input_parser(raw.trim()).unwrap().1;
        assert_eq!(solve1(&parsed), 5);
    }
}
