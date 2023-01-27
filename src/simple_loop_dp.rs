use std::cmp::{max, min};

// ref: https://en.wikipedia.org/wiki/Dynamic_programming#Egg_dropping_puzzle
#[allow(non_snake_case)]
pub fn simple_loop_dp(N: usize, K: usize) -> i32 {
    // K: width, N: height in the dp table to match dp[n][k] to W(n,k) in Wikipedia.
    let mut dp = vec![vec![0_i32; K + 1]; N + 1];
    for i in 0..=K {
        dp[1][i] = i as i32;
    }
    // items in (n < 2 || k < 1) are already calculated

    for n in 2..=N {
        for k in 1..=K {
            let mut minval = i32::MAX;
            for x in 1..=k {
                minval = min(minval, max(dp[n][k - x], dp[n - 1][x - 1]));
            }
            dp[n][k] = 1 + minval;
        }
    }
    dp[N][K]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::egg_drop;

    #[test]
    fn test_simple_loop_dp() {
        assert_eq!(egg_drop(simple_loop_dp, 2, 1), 1);
        assert_eq!(egg_drop(simple_loop_dp, 1, 2), 2);
        assert_eq!(egg_drop(simple_loop_dp, 2, 6), 3);
        assert_eq!(egg_drop(simple_loop_dp, 3, 14), 4);
        assert_eq!(egg_drop(simple_loop_dp, 4, 30), 5);
        assert_eq!(egg_drop(simple_loop_dp, 5, 62), 6);
        assert_eq!(egg_drop(simple_loop_dp, 6, 126), 7);
        assert_eq!(egg_drop(simple_loop_dp, 4, 2), 2);
        assert_eq!(egg_drop(simple_loop_dp, 8, 8), 4);
        assert_eq!(egg_drop(simple_loop_dp, 50, 500), 9);
    }
}
