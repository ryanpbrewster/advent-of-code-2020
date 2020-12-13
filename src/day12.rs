enum Move {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

struct Ship {
    x: i32,
    y: i32,
    theta: i32, // in degrees
}
impl Ship {
    fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            theta: 0,
        }
    }
    fn step(&mut self, m: Move, n: i32) {
        match m {
            Move::North => self.y += n,
            Move::South => self.y -= n,
            Move::East => self.x += n,
            Move::West => self.x -= n,
            Move::Left => self.theta += n,
            Move::Right => self.theta -= n,
            Move::Forward => match self.theta {
                0 => self.x += n,
                90 => self.y += n,
                180 => self.x -= n,
                270 => self.y -= n,
                _ => panic!("invalid angle: {}", self.theta),
            },
        }
        self.theta = ((self.theta % 360) + 360) % 360;
    }
}

#[cfg(test)]
mod test {
    use super::{Move, Ship};

    const SMALL: &str = r" F10 N3 F7 R90 F11 ";
    fn parse(input: &str) -> Vec<(Move, i32)> {
        input
            .trim()
            .split_ascii_whitespace()
            .map(|s| {
                let m = match &s[0..1] {
                    "N" => Move::North,
                    "S" => Move::South,
                    "E" => Move::East,
                    "W" => Move::West,
                    "F" => Move::Forward,
                    "L" => Move::Left,
                    "R" => Move::Right,
                    _ => panic!("invalid move: {}", s),
                };
                let n = s[1..].parse().unwrap();
                (m, n)
            })
            .collect()
    }
    #[test]
    fn small1() {
        let input = parse(SMALL);
        let mut ship = Ship::new();
        for (m, n) in input {
            ship.step(m, n);
        }
        assert_eq!(ship.x.abs() + ship.y.abs(), 25);
    }

    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day12.input").unwrap();
        let input = parse(&raw);
        let mut ship = Ship::new();
        for (m, n) in input {
            ship.step(m, n);
        }
        assert_eq!(ship.x.abs() + ship.y.abs(), 439);
    }
}
