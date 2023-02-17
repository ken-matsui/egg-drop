use std::cmp::{max, min};

// ref: https://en.wikipedia.org/wiki/Dynamic_programming#Egg_dropping_puzzle
pub fn rec(n: usize, k: usize) -> i32 {
    if n == 1 || k == 0 || k == 1 {
        return k as i32;
    }

    let mut minval = i32::MAX;
    for x in 1..=k {
        minval = min(minval, 1 + max(rec(n, k - x), rec(n - 1, x - 1)));
    }
    minval
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::egg_drop_old;

    #[test]
    fn test_rec() {
        assert_eq!(egg_drop_old(rec, 2, 1), 1);
        assert_eq!(egg_drop_old(rec, 1, 2), 2);
        assert_eq!(egg_drop_old(rec, 2, 6), 3);
        assert_eq!(egg_drop_old(rec, 3, 14), 4);
    }
}
