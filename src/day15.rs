fn solve(input: &str, n: usize) -> usize {
    let xs: Vec<usize> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let mut d = vec![0; n];
    for (idx, &x) in xs.iter().enumerate() {
        d[x] = idx + 1;
    }
    let mut cur = xs[xs.len() - 1];
    for i in xs.len()..n {
        let prev = d[cur];
        d[cur] = i;
        cur = if prev == 0 { 0 } else { i - prev };
    }
    cur
}

#[cfg(test)]
mod test {
    use super::solve;

    #[test]
    fn small1() {
        assert_eq!(solve("0,3,6", 2020), 436);
    }

    #[test]
    fn normal1() {
        assert_eq!(solve("1,20,11,6,12,0", 2020), 1085);
    }

    #[test]
    fn normal2() {
        assert_eq!(solve("1,20,11,6,12,0", 30_000_000), 10652);
    }
}
