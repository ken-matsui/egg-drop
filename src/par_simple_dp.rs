use std::cmp::{max, min};

/// requires:
/// 1. dp[n][0] = 0 forall n s.t. n >= 0
/// 2. dp[1][k] = k forall k s.t. k >= 0
fn compute_block(dp: &mut Vec<Vec<i32>>, from_n: usize, to_n: usize, from_k: usize, to_k: usize) {
    for n in from_n..=to_n {
        for k in from_k..=to_k {
            if n < 2 || k < 1 {
                // already calculated
                continue;
            }

            let mut minval = i32::MAX;
            for x in 1..=k {
                minval = min(minval, max(dp[n - 1][x - 1], dp[n][k - x]));
            }
            dp[n][k] = 1 + minval;
        }
    }
}

// ref: https://en.wikipedia.org/wiki/Dynamic_programming#Egg_dropping_puzzle
#[allow(non_snake_case)]
pub fn par_simple_dp(N: usize, K: usize) -> i32 {
    // K: width, N: height in the dp table to match dp[n][k] to W(n,k) in Wikipedia.
    let mut dp = vec![vec![0_i32; K + 1]; N + 1];
    for k in 0..=K {
        dp[1][k] = k as i32;
    }
    // items in (n < 2 || k < 1) are already calculated

    let step = 3; // will be 3*3 blocks
    for u in (2..=(N + K)).step_by(step) {
        for k in (0..=u).step_by(step) {
            let n = u - k;
            if n <= N && k <= K {
                let to_n = if n + step - 1 < N { n + step - 1 } else { N };
                let to_k = if k + step - 1 < K { k + step - 1 } else { K };
                println!("({n}, {k})..=({to_n}, {to_k})");
                compute_block(&mut dp, n, to_n, k, to_k);
            }
        }
        println!();
    }

    #[allow(clippy::needless_range_loop)]
    for n in 0..=N {
        for k in 0..=K {
            print!("{} ", dp[n][k]);
        }
        println!();
    }

    dp[N][K]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::egg_drop;

    #[test]
    fn test_par_simple_dp() {
        assert_eq!(egg_drop(par_simple_dp, 8, 8), 4); // useful for debugging
        assert_eq!(egg_drop(par_simple_dp, 2, 1), 1);
        assert_eq!(egg_drop(par_simple_dp, 1, 2), 2);
        assert_eq!(egg_drop(par_simple_dp, 2, 6), 3);
        assert_eq!(egg_drop(par_simple_dp, 3, 14), 4);
        assert_eq!(egg_drop(par_simple_dp, 4, 30), 5);
        assert_eq!(egg_drop(par_simple_dp, 5, 62), 6);
        assert_eq!(egg_drop(par_simple_dp, 6, 126), 7);
        assert_eq!(egg_drop(par_simple_dp, 50, 500), 9);
        assert_eq!(egg_drop(par_simple_dp, 10, 8), 4);
        assert_eq!(egg_drop(par_simple_dp, 50, 500), 9);
    }
}
