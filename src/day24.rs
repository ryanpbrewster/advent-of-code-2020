use std::collections::{HashMap, HashSet};

use nom::{branch::alt, bytes::complete::tag, combinator::value, multi::many1, IResult};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}
type Path = Vec<Direction>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coordinate(i32, i32);

fn shift(Coordinate(p, q): Coordinate, dir: Direction) -> Coordinate {
    match dir {
        Direction::E => Coordinate(p + 1, q),
        Direction::SE => Coordinate(p, q + 1),
        Direction::SW => Coordinate(p - 1, q + 1),
        Direction::W => Coordinate(p - 1, q),
        Direction::NW => Coordinate(p, q - 1),
        Direction::NE => Coordinate(p + 1, q - 1),
    }
}
fn traverse(origin: Coordinate, path: &Path) -> Coordinate {
    path.into_iter().fold(origin, |p, &dir| shift(p, dir))
}
fn neighbors(Coordinate(p, q): Coordinate) -> Vec<Coordinate> {
    vec![
        Coordinate(p + 1, q),
        Coordinate(p, q + 1),
        Coordinate(p - 1, q + 1),
        Coordinate(p - 1, q),
        Coordinate(p, q - 1),
        Coordinate(p + 1, q - 1),
    ]
}

fn path_parser(input: &str) -> IResult<&str, Path> {
    many1(direction_parser)(input)
}
fn direction_parser(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::E, tag("e")),
        value(Direction::SE, tag("se")),
        value(Direction::SW, tag("sw")),
        value(Direction::W, tag("w")),
        value(Direction::NW, tag("nw")),
        value(Direction::NE, tag("ne")),
    ))(input)
}

fn solve1(paths: &[Path]) -> usize {
    let mut flipped = HashSet::new();
    for path in paths {
        let tile = traverse(Coordinate(0, 0), path);
        if flipped.contains(&tile) {
            flipped.remove(&tile);
        } else {
            flipped.insert(tile);
        }
    }
    flipped.len()
}

fn solve2(paths: &[Path]) -> usize {
    let mut flipped = HashSet::new();
    for path in paths {
        let tile = traverse(Coordinate(0, 0), path);
        if flipped.contains(&tile) {
            flipped.remove(&tile);
        } else {
            flipped.insert(tile);
        }
    }
    for _ in 0..100 {
        flipped = evolve(flipped);
    }
    flipped.len()
}
fn evolve(black_tiles: HashSet<Coordinate>) -> HashSet<Coordinate> {
    let mut black_neighbors: HashMap<Coordinate, usize> = HashMap::new();
    for &tile in &black_tiles {
        for n in neighbors(tile) {
            *black_neighbors.entry(n).or_default() += 1;
        }
    }
    let mut next = HashSet::new();
    for (tile, neighbor_count) in black_neighbors {
        let is_black = black_tiles.contains(&tile);
        let stay_black = is_black && neighbor_count == 1 || neighbor_count == 2;
        let flip_white = !is_black && neighbor_count == 2;
        if stay_black || flip_white {
            next.insert(tile);
        }
    }
    next
}

#[cfg(test)]
mod test {
    use super::{path_parser, solve1, solve2, Direction, Path};

    #[test]
    fn parser_test() {
        assert_eq!(
            path_parser("ese").unwrap().1,
            vec![Direction::E, Direction::SE]
        );
    }

    fn parse_paths(input: &str) -> Vec<Path> {
        input
            .trim()
            .lines()
            .map(|line| path_parser(line.trim()).unwrap().1)
            .collect()
    }

    const SMALL: &str = r"
        sesenwnenenewseeswwswswwnenewsewsw
        neeenesenwnwwswnenewnwwsewnenwseswesw
        seswneswswsenwwnwse
        nwnwneseeswswnenewneswwnewseswneseene
        swweswneswnenwsewnwneneseenw
        eesenwseswswnenwswnwnwsewwnwsene
        sewnenenenesenwsewnenwwwse
        wenwwweseeeweswwwnwwe
        wsweesenenewnwwnwsenewsenwwsesesenwne
        neeswseenwwswnwswswnw
        nenwswwsewswnenenewsenwsenwnesesenew
        enewnwewneswsewnwswenweswnenwsenwsw
        sweneswneswneneenwnewenewwneswswnese
        swwesenesewenwneswnwwneseswwne
        enesenwswwswneneswsenwnewswseenwsese
        wnwnesenesenenwwnenwsewesewsesesew
        nenewswnwewswnenesenwnesewesw
        eneswnwswnwsenenwnwnwwseeswneewsenese
        neswnwewnwnwseenwseesewsenwsweewe
        wseweeenwnesenwwwswnew
    ";

    #[test]
    fn small1() {
        let paths: Vec<Path> = parse_paths(SMALL);
        assert_eq!(solve1(&paths), 10);
    }

    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day24.input").unwrap();
        let paths: Vec<Path> = parse_paths(&raw);
        assert_eq!(solve1(&paths), 488);
    }

    #[test]
    fn small2() {
        let paths: Vec<Path> = parse_paths(SMALL);
        assert_eq!(solve2(&paths), 2208);
    }

    #[test]
    fn normal2() {
        let raw = std::fs::read_to_string("data/day24.input").unwrap();
        let paths: Vec<Path> = parse_paths(&raw);
        assert_eq!(solve2(&paths), 4118);
    }
}
