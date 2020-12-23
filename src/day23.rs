use std::collections::HashMap;

struct Ring {
    next: HashMap<i32, i32>,
    cur: i32,
}
impl Ring {
    fn to_vec(&self, init: i32) -> Vec<i32> {
        let mut xs = vec![init];
        let mut cur = self.next[&init];
        while cur != init {
            xs.push(cur);
            cur = self.next[&cur];
        }
        xs
    }
    fn step(&mut self) {
        let snip0 = self.next[&self.cur];
        let snip1 = self.next[&snip0];
        let snip2 = self.next[&snip1];
        let neighbor = self.next[&snip2];
        let dest = {
            let mut target = self.cur - 1;
            if target < 1 {
                target = self.next.len() as i32;
            }
            while target == snip0 || target == snip1 || target == snip2 {
                target -= 1;
                if target < 1 {
                    target = self.next.len() as i32;
                }
            }
            target
        };
        let dest1 = self.next[&dest];

        *self.next.get_mut(&self.cur).unwrap() = neighbor;
        *self.next.get_mut(&dest).unwrap() = snip0;
        *self.next.get_mut(&snip2).unwrap() = dest1;
        self.cur = neighbor;
    }
}
impl From<&[i32]> for Ring {
    fn from(xs: &[i32]) -> Ring {
        let mut prev = *xs.last().unwrap();
        let mut next = HashMap::new();
        for &cur in xs {
            next.insert(prev, cur);
            prev = cur;
        }
        Ring { next, cur: xs[0] }
    }
}

#[cfg(test)]
mod test {
    use super::Ring;

    #[test]
    fn ring_test() {
        let ring = Ring::from([3, 8, 9, 1, 2, 5, 4, 6, 7].as_ref());
        assert_eq!(ring.to_vec(1), vec![1, 2, 5, 4, 6, 7, 3, 8, 9]);
    }

    #[test]
    fn small1() {
        let mut ring = Ring::from([3, 8, 9, 1, 2, 5, 4, 6, 7].as_ref());
        for _ in 0..100 {
            ring.step();
        }
        assert_eq!(ring.to_vec(1), vec![1, 6, 7, 3, 8, 4, 5, 2, 9]);
    }

    #[test]
    fn normal1() {
        let mut ring: Ring = Ring::from([4, 9, 6, 1, 3, 8, 5, 2, 7].as_ref());
        for _ in 0..100 {
            ring.step();
        }
        assert_eq!(ring.to_vec(1), vec![1, 6, 9, 4, 2, 5, 8, 3, 7]);
    }

    #[test]
    fn normal2() {
        let mut xs = vec![4, 9, 6, 1, 3, 8, 5, 2, 7];
        for i in 10..=1_000_000 {
            xs.push(i);
        }
        let mut ring = Ring::from(xs.as_ref());
        for _ in 0..10_000_000 {
            ring.step();
        }

        let n1 = ring.next[&1];
        let n2 = ring.next[&n1];
        assert_eq!(n1 as i64 * n2 as i64, 218882971435);
    }
}
