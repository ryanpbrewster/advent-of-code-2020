fn get_seat_number(pass: &str) -> Option<usize> {
    let mut acc = 0;
    for ch in pass.chars() {
        let v = match ch {
            'F' | 'L' => 0,
            'B' | 'R' => 1,
            _ => return None,
        };
        acc = 2 * acc + v;
    }
    Some(acc)
}

// Find the first contiguous missing value from `xs`
fn find_missing(xs: &[usize]) -> Option<usize> {
    if xs.is_empty() {
        return None;
    }
    let mut cur = xs[0];
    for &x in &xs[1..] {
        if x != cur + 1 {
            return Some(cur + 1);
        }
        cur = x;
    }
    None
}

#[cfg(test)]
mod test {
    use super::{find_missing, get_seat_number};

    #[test]
    fn small1() {
        assert_eq!(get_seat_number("BFFFBBFRRR").unwrap(), 567);
        assert_eq!(get_seat_number("FFFBBBFRRR").unwrap(), 119);
        assert_eq!(get_seat_number("BBFFBBFRLL").unwrap(), 820);
    }

    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day05.input").unwrap();
        let seat_numbers: Vec<usize> = raw
            .lines()
            .map(|line| get_seat_number(line.trim()).unwrap())
            .collect();
        assert_eq!(seat_numbers.into_iter().max().unwrap(), 806);
    }

    #[test]
    fn normal2() {
        let raw = std::fs::read_to_string("data/day05.input").unwrap();
        let mut seat_numbers: Vec<usize> = raw
            .lines()
            .map(|line| get_seat_number(line.trim()).unwrap())
            .collect();
        seat_numbers.sort();
        assert_eq!(find_missing(&seat_numbers).unwrap(), 562);
    }
}
