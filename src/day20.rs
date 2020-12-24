use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

struct Tile {
    id: usize,
    rows: Vec<Vec<u8>>,
}
impl Tile {
    fn top(&self) -> Vec<u8> {
        self.rows.first().unwrap().clone()
    }
    fn bottom(&self) -> Vec<u8> {
        self.rows.last().unwrap().clone()
    }
    fn left(&self) -> Vec<u8> {
        self.rows
            .iter()
            .map(|r| r.first().unwrap())
            .copied()
            .collect()
    }
    fn right(&self) -> Vec<u8> {
        self.rows
            .iter()
            .map(|r| r.last().unwrap())
            .copied()
            .collect()
    }
}

lazy_static! {
    static ref TILE_ID: Regex = Regex::new("^Tile (\\d+):$").unwrap();
}
impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines().map(|l| l.trim());
        let title = lines.next().ok_or(())?;
        let id_raw = TILE_ID.captures(title).ok_or(())?.get(1).unwrap().as_str();
        let id = id_raw.parse().map_err(|_| ())?;

        let rows = lines
            .map(|l| l.bytes().map(|b| if b == b'.' { 0 } else { 1 }).collect())
            .collect();

        Ok(Tile { id, rows })
    }
}

fn solve1(input: &str) -> usize {
    let tiles: HashMap<usize, Tile> = input
        .trim()
        .split("\n\n")
        .map(|t| {
            let tile: Tile = t.parse().unwrap();
            (tile.id, tile)
        })
        .collect();
    let mut by_edge: HashMap<Vec<u8>, Vec<usize>> = HashMap::new();
    for (&id, tile) in &tiles {
        let mut left = tile.left();
        let mut right = tile.right();
        let mut top = tile.top();
        let mut bottom = tile.bottom();
        by_edge.entry(left.clone()).or_default().push(id);
        by_edge.entry(right.clone()).or_default().push(id);
        by_edge.entry(top.clone()).or_default().push(id);
        by_edge.entry(bottom.clone()).or_default().push(id);

        left.reverse();
        right.reverse();
        top.reverse();
        bottom.reverse();

        by_edge.entry(left).or_default().push(id);
        by_edge.entry(right).or_default().push(id);
        by_edge.entry(top).or_default().push(id);
        by_edge.entry(bottom).or_default().push(id);
    }

    let mut corners = Vec::new();
    for (&id, tile) in &tiles {
        let left = by_edge[&tile.left()].len();
        let right = by_edge[&tile.right()].len();
        let top = by_edge[&tile.top()].len();
        let bottom = by_edge[&tile.bottom()].len();
        println!("{} -> ({}, {}, {}, {})", id, left, right, top, bottom);
        if left + right + top + bottom == 6 {
            corners.push(id);
        }
    }
    corners.iter().copied().product()
}

#[cfg(test)]
mod test {
    use super::{solve1, Tile};

    #[test]
    fn parser_test() {
        let raw = r"
            Tile 2311:
            ..##.#..#.
            ##..#.....
            #...##..#.
            ####.#...#
            ##.##.###.
            ##...#.###
            .#.#.#..##
            ..#....#..
            ###...#.#.
            ..###..###
        ";
        let tile = raw.parse::<Tile>().unwrap();
        assert_eq!(tile.id, 2311);
        assert_eq!(tile.rows.len(), 10);
    }

    const SMALL: &str = r"
        Tile 2311:
        ..##.#..#.
        ##..#.....
        #...##..#.
        ####.#...#
        ##.##.###.
        ##...#.###
        .#.#.#..##
        ..#....#..
        ###...#.#.
        ..###..###

        Tile 1951:
        #.##...##.
        #.####...#
        .....#..##
        #...######
        .##.#....#
        .###.#####
        ###.##.##.
        .###....#.
        ..#.#..#.#
        #...##.#..

        Tile 1171:
        ####...##.
        #..##.#..#
        ##.#..#.#.
        .###.####.
        ..###.####
        .##....##.
        .#...####.
        #.##.####.
        ####..#...
        .....##...

        Tile 1427:
        ###.##.#..
        .#..#.##..
        .#.##.#..#
        #.#.#.##.#
        ....#...##
        ...##..##.
        ...#.#####
        .#.####.#.
        ..#..###.#
        ..##.#..#.

        Tile 1489:
        ##.#.#....
        ..##...#..
        .##..##...
        ..#...#...
        #####...#.
        #..#.#.#.#
        ...#.#.#..
        ##.#...##.
        ..##.##.##
        ###.##.#..

        Tile 2473:
        #....####.
        #..#.##...
        #.##..#...
        ######.#.#
        .#...#.#.#
        .#########
        .###.#..#.
        ########.#
        ##...##.#.
        ..###.#.#.

        Tile 2971:
        ..#.#....#
        #...###...
        #.#.###...
        ##.##..#..
        .#####..##
        .#..####.#
        #..#.#..#.
        ..####.###
        ..#.#.###.
        ...#.#.#.#

        Tile 2729:
        ...#.#.#.#
        ####.#....
        ..#.#.....
        ....#..#.#
        .##..##.#.
        .#.####...
        ####.#.#..
        ##.####...
        ##..#.##..
        #.##...##.

        Tile 3079:
        #.#.#####.
        .#..######
        ..#.......
        ######....
        ####.#..#.
        .#...#.##.
        #.#####.##
        ..#.###...
        ..#.......
        ..#.###...
    ";

    #[test]
    fn small1() {
        assert_eq!(solve1(SMALL), 20899048083289);
    }

    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day20.input").unwrap();
        assert_eq!(solve1(&raw), 18449208814679);
    }
}
