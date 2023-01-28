use std::cmp::{max, min};

use crate::dptable::{self, get, insert, DpTable};

/// requires:
/// 1. dp[n][0] = 0 forall n s.t. n >= 0
/// 2. dp[1][k] = k forall k s.t. k >= 0
/// 3. 1, 2 means dp[n][k] = already calculated forall n s.t. n < 2 or forall k s.t. k < 1
pub(crate) fn compute_block(
    dp: &DpTable<usize, i32>,
    from_n: usize,
    to_n: usize,
    from_k: usize,
    to_k: usize,
) {
    for n in from_n..=to_n {
        if n < 2 {
            // already calculated
            continue;
        }
        for k in from_k..=to_k {
            if k < 1 {
                // already calculated
                continue;
            }

            let mut minval = i32::MAX;
            for x in 1..=k {
                minval = min(minval, max(get(dp, n - 1, x - 1), get(dp, n, k - x)));
            }
            insert(dp, n, k, 1 + minval);
        }
    }
}

// ref: https://en.wikipedia.org/wiki/Dynamic_programming#Egg_dropping_puzzle
#[allow(non_snake_case)]
pub fn simple_dp(N: usize, K: usize) -> i32 {
    let dp = dptable::new(N, K);

    let step = 3; // step*step sized block
    for u in (2..=(N + K)).step_by(step) {
        for k in (0..=u).step_by(step) {
            let n = u - k;
            if n <= N && k <= K {
                let to_n = if n + step - 1 < N { n + step - 1 } else { N };
                let to_k = if k + step - 1 < K { k + step - 1 } else { K };
                println!("({n}, {k})..=({to_n}, {to_k})");
                compute_block(&dp, n, to_n, k, to_k);
            }
        }
        println!();
    }
    dptable::print(&dp, N, K);

    get(&dp, N, K)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::egg_drop;

    #[test]
    fn test_simple_dp() {
        assert_eq!(egg_drop(simple_dp, 8, 8), 4); // useful for debugging
        assert_eq!(egg_drop(simple_dp, 2, 1), 1);
        assert_eq!(egg_drop(simple_dp, 1, 2), 2);
        assert_eq!(egg_drop(simple_dp, 2, 6), 3);
        assert_eq!(egg_drop(simple_dp, 3, 14), 4);
        assert_eq!(egg_drop(simple_dp, 4, 30), 5);
        assert_eq!(egg_drop(simple_dp, 5, 62), 6);
        assert_eq!(egg_drop(simple_dp, 6, 126), 7);
        assert_eq!(egg_drop(simple_dp, 50, 500), 9);
        assert_eq!(egg_drop(simple_dp, 10, 8), 4);
        assert_eq!(egg_drop(simple_dp, 50, 500), 9);
    }
}
