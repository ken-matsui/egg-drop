use std::cmp::{max, min};

// ref: https://en.wikipedia.org/wiki/Dynamic_programming#Egg_dropping_puzzle
pub fn dp(n: i32, h: i32) -> i32 {
    #[allow(non_snake_case)]
    let N = n as usize;
    #[allow(non_snake_case)]
    let H = h as usize;

    let mut numdrops = vec![vec![0_i32; H + 1]; N + 1];

    for i in 0..=h {
        numdrops[1][i as usize] = i;
    }

    for i in 2..=N {
        for j in 1..=H {
            let mut minval = i32::MAX;
            for x in 1..=j {
                minval = min(minval, 1 + max(numdrops[i][j - x], numdrops[i - 1][x - 1]));
            }
            numdrops[i][j] = minval;
        }
    }
    numdrops[N][H]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::egg_drop;

    #[test]
    fn test_dp() {
        assert_eq!(egg_drop(dp, 2, 1), 1);
        assert_eq!(egg_drop(dp, 1, 2), 2);
        assert_eq!(egg_drop(dp, 2, 6), 3);
        assert_eq!(egg_drop(dp, 3, 14), 4);
        assert_eq!(egg_drop(dp, 4, 30), 5);
        assert_eq!(egg_drop(dp, 5, 62), 6);
        assert_eq!(egg_drop(dp, 6, 126), 7);
        assert_eq!(egg_drop(dp, 50, 500), 9);
    }
}
