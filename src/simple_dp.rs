use std::cmp::{max, min};

use debug_print::debug_println as dprintln;

use crate::dptable::{DpTable, DpTablePtr};

/// requires:
/// 1. dp[n][0] = 0 forall n s.t. n >= 0
/// 2. dp[1][k] = k forall k s.t. k >= 0
/// 3. 1, 2 means dp[n][k] = already calculated forall n s.t. n < 2 or forall k s.t. k < 1
pub(crate) fn compute_block(
    dp: DpTablePtr<i32>,
    from_n: usize,
    to_n: usize,
    from_k: usize,
    to_k: usize,
) {
    // We assume these are already calculated
    let from_n = if from_n < 2 { 2 } else { from_n };
    let from_k = if from_k < 1 { 1 } else { from_k };

    for k in from_k..=to_k {
        for n in from_n..=to_n {
            let mut minval = i32::MAX;
            for x in 1..=k {
                unsafe {
                    minval = min(minval, max(dp.get(n - 1, x - 1), dp.get(n, k - x)));
                }
            }
            unsafe {
                dp.insert(n, k, 1 + minval);
            }
        }
    }
}

// ref: https://en.wikipedia.org/wiki/Dynamic_programming#Egg_dropping_puzzle
#[allow(non_snake_case)]
// bsize*bsize sized block
pub fn simple_dp(N: usize, K: usize, bsize: usize) -> i32 {
    let mut dp = DpTable::new(N, K);
    let dp_p = dp.as_mut_ptr();

    let mut u = 2;
    while u <= N + K {
        let mut k = 0;
        while k <= u {
            let n = u - k;
            if n <= N && k <= K {
                let to_n = if n + bsize - 1 < N { n + bsize - 1 } else { N };
                let to_k = if k + bsize - 1 < K { k + bsize - 1 } else { K };
                dprintln!("({n}, {k})..=({to_n}, {to_k})");
                compute_block(dp_p.clone(), n, to_n, k, to_k);
            }

            k += bsize; // step_by
        }
        dprintln!();

        u += bsize; // step_by
    }
    dprintln!("{:?}", dp);
    dp.get(N, K)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::egg_drop;

    #[test]
    fn test_simple_dp() {
        assert_eq!(egg_drop(simple_dp, 8, 8, 2), 4); // useful for debugging
        assert_eq!(egg_drop(simple_dp, 2, 1, 2), 1);
        assert_eq!(egg_drop(simple_dp, 1, 2, 2), 2);
        assert_eq!(egg_drop(simple_dp, 2, 6, 2), 3);
        assert_eq!(egg_drop(simple_dp, 3, 14, 2), 4);
        assert_eq!(egg_drop(simple_dp, 4, 30, 2), 5);
        assert_eq!(egg_drop(simple_dp, 5, 62, 2), 6);
        assert_eq!(egg_drop(simple_dp, 6, 126, 2), 7);
        assert_eq!(egg_drop(simple_dp, 50, 500, 2), 9);
        assert_eq!(egg_drop(simple_dp, 10, 8, 2), 4);
        assert_eq!(egg_drop(simple_dp, 50, 500, 2), 9);
    }
}
