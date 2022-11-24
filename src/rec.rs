use std::cmp::{max, min};

// ref: https://en.wikipedia.org/wiki/Dynamic_programming#Egg_dropping_puzzle
pub fn rec(n: i32, h: i32) -> i32 {
    if n == 1 || h == 0 || h == 1 {
        return h;
    }

    let mut minval = i32::MAX;
    for x in 1..=h {
        minval = min(minval, 1 + max(rec(n, h - x), rec(n - 1, x - 1)));
    }
    minval
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::egg_drop;

    #[test]
    fn test_rec() {
        assert_eq!(egg_drop(rec, 2, 1), 1);
        assert_eq!(egg_drop(rec, 1, 2), 2);
        assert_eq!(egg_drop(rec, 2, 6), 3);
        assert_eq!(egg_drop(rec, 3, 14), 4);
    }
}
