fn count_trees(grid: &str, vx: usize, vy: usize) -> usize {
    let lines: Vec<String> = grid.split_ascii_whitespace().map(String::from).collect();
    let width = lines[0].len();
    let mut count = 0;
    let mut row = 0;
    let mut col = 0;
    while row < lines.len() {
        if lines[row].chars().nth(col).unwrap() == '#' {
            count += 1;
        }
        row = row + vy;
        col = (col + vx) % width;
    }
    count
}

#[cfg(test)]
mod test {
    use super::count_trees;

    #[test]
    fn small1() {
        let raw = r"
            ..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#
        ";
        let count = count_trees(raw, 3, 1);
        assert_eq!(count, 7);
    }

    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day03.input").unwrap();
        let count = count_trees(&raw, 3, 1);
        assert_eq!(count, 214);
    }

    #[test]
    fn normal2() {
        let raw = std::fs::read_to_string("data/day03.input").unwrap();
        let v11 = count_trees(&raw, 1, 1);
        let v31 = count_trees(&raw, 3, 1);
        let v51 = count_trees(&raw, 5, 1);
        let v71 = count_trees(&raw, 7, 1);
        let v12 = count_trees(&raw, 1, 2);
        assert_eq!(v11 * v31 * v51 * v71 * v12, 8336352024);
    }
}
