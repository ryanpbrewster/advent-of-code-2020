const MAGIC: u64 = 20201227;
struct Transform {
    subject: u64,
    cur: u64,
}
impl Transform {
    fn new(subject: u64) -> Transform {
        Transform { subject, cur: 1 }
    }
}
impl Iterator for Transform {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.cur;
        self.cur = (self.cur * self.subject) % MAGIC;
        Some(ret)
    }
}

fn solve1(card: u64, door: u64) -> u64 {
    let card_loop = Transform::new(7)
        .enumerate()
        .find(|&(_, v)| v == card)
        .unwrap()
        .0;
    Transform::new(door).nth(card_loop as usize).unwrap()
}
#[cfg(test)]
mod test {
    use super::solve1;

    #[test]
    fn small1() {
        let card = 5764801;
        let door = 17807724;
        assert_eq!(solve1(card, door), 14897079);
    }

    #[test]
    fn normal1() {
        let card = 12090988;
        let door = 240583;
        assert_eq!(solve1(card, door), 3015200);
    }
}
