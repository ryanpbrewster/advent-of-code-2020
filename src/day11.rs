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
            .filter(|&p| grid.get(p) == Some(&Cell::Occupied))
            .count();
        let cur = *grid.get(pos).unwrap();
        if cur == Cell::Vacant && occupied == 0 {
            Cell::Occupied
        } else if cur == Cell::Occupied && occupied >= 4 {
            Cell::Vacant
        } else {
            cur
        }
    })
}

fn step2(grid: &Grid<Cell>) -> Grid<Cell> {
    Grid::from_fn(grid.width, grid.height, |pos| {
        let occupied = rays(pos)
            .into_iter()
            .map(|r| {
                let visible = r.map(|p| grid.get(p)).find(|&c| c != Some(&Cell::Floor));
                if visible == Some(Some(&Cell::Occupied)) {
                    1
                } else {
                    0
                }
            })
            .sum::<usize>();
        let cur = *grid.get(pos).unwrap();
        if cur == Cell::Vacant && occupied == 0 {
            Cell::Occupied
        } else if cur == Cell::Occupied && occupied >= 5 {
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

fn rays((i, j): Pos) -> Vec<impl Iterator<Item = Pos>> {
    let ray = |di, dj| (1..).map(move |k| (i + k * di, j + k * dj));
    vec![
        ray(-1, -1),
        ray(-1, 0),
        ray(-1, 1),
        ray(0, -1),
        ray(0, 1),
        ray(1, -1),
        ray(1, 0),
        ray(1, 1),
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

fn stabilize2(mut grid: Grid<Cell>) -> Grid<Cell> {
    loop {
        let next = step2(&grid);
        if grid == next {
            return grid;
        }
        grid = next;
    }
}

#[cfg(test)]
mod test {
    use crate::grid::Grid;

    use super::{stabilize, stabilize2, Cell};

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

    #[test]
    fn normal2() {
        let raw = std::fs::read_to_string("data/day11.input").unwrap();
        let grid = parse(&raw);
        let stable = stabilize2(grid);
        let occupied = stable
            .items
            .into_iter()
            .filter(|&c| c == Cell::Occupied)
            .count();
        assert_eq!(occupied, 2013);
    }
}
