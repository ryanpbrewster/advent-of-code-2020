use std::fmt;

use crate::grid::{Grid, Pos};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Cell {
    Occupied,
    Vacant,
    Floor,
}
impl Default for Cell {
    fn default() -> Self {
        Cell::Floor
    }
}
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Cell::Floor => ".",
            Cell::Occupied => "#",
            Cell::Vacant => "L",
        })
    }
}

fn step(grid: &Grid<Cell>) -> Grid<Cell> {
    Grid::from_fn(grid.width, grid.height, |pos| {
        let occupied = neighbors(pos)
            .into_iter()
            .filter(|&p| grid[p] == Cell::Occupied)
            .count();
        let cur = grid[pos];
        if cur == Cell::Vacant && occupied == 0 {
            Cell::Occupied
        } else if cur == Cell::Occupied && occupied >= 4 {
            Cell::Vacant
        } else {
            cur
        }
    })
}

fn neighbors((i, j): Pos) -> Vec<Pos> {
    vec![
        (i - 1, j - 1),
        (i - 1, j),
        (i - 1, j + 1),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
    ]
}

fn stabilize(mut grid: Grid<Cell>) -> Grid<Cell> {
    loop {
        let next = step(&grid);
        if grid == next {
            return grid;
        }
        grid = next;
    }
}
#[cfg(test)]
mod test {
    use crate::grid::Grid;

    use super::{stabilize, Cell};

    const SMALL: &str = r"
        L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL
    ";

    fn parse(input: &str) -> Grid<Cell> {
        let lines: Vec<&str> = input
            .lines()
            .map(|l| l.trim())
            .filter(|line| !line.is_empty())
            .collect();
        Grid::new(
            lines[0].len(),
            lines.len(),
            lines
                .into_iter()
                .flat_map(|l| l.chars())
                .map(|c| match c {
                    'L' => Cell::Vacant,
                    '#' => Cell::Occupied,
                    '.' => Cell::default(),
                    _ => panic!("illegal char"),
                })
                .collect(),
        )
    }
    #[test]
    fn small1() {
        let grid = parse(SMALL);
        let stable = stabilize(grid);
        let occupied = stable
            .items
            .into_iter()
            .filter(|&c| c == Cell::Occupied)
            .count();
        assert_eq!(occupied, 37);
    }

    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day11.input").unwrap();
        let grid = parse(&raw);
        let stable = stabilize(grid);
        let occupied = stable
            .items
            .into_iter()
            .filter(|&c| c == Cell::Occupied)
            .count();
        assert_eq!(occupied, 2238);
    }
}
