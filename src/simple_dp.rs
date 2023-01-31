use std::cmp::{max, min};
use std::sync::{Arc, RwLock};

use debug_print::debug_println as dprintln;

use crate::dptable::DpTable;

/// requires:
/// 1. dp[n][0] = 0 forall n s.t. n >= 0
/// 2. dp[1][k] = k forall k s.t. k >= 0
/// 3. 1, 2 means dp[n][k] = already calculated forall n s.t. n < 2 or forall k s.t. k < 1
pub(crate) fn compute_block(
    dp: Arc<RwLock<DpTable<i32>>>,
    from_n: usize,
    to_n: usize,
    from_k: usize,
    to_k: usize,
) {
    // We assume these are already calculated
    let from_n = if from_n < 2 { 2 } else { from_n };
    let from_k = if from_k < 1 { 1 } else { from_k };

    for n in from_n..=to_n {
        for k in from_k..=to_k {
            let mut minval = i32::MAX;

            {
                let dp_reader = dp.read().unwrap();
                for x in 1..=k {
                    minval = min(
                        minval,
                        max(dp_reader.get(n - 1, x - 1), dp_reader.get(n, k - x)),
                    );
                }
            }
            {
                let mut dp_writer = dp.write().unwrap();
                dp_writer.insert(n, k, 1 + minval);
            }
        }
    }
}

// ref: https://en.wikipedia.org/wiki/Dynamic_programming#Egg_dropping_puzzle
#[allow(non_snake_case)]
pub fn simple_dp(N: usize, K: usize) -> i32 {
    let dp = Arc::new(RwLock::new(DpTable::new(N, K)));

    let block = 100; // block*block sized block
    for u in (2..=(N + K)).step_by(block) {
        for k in (0..=u).step_by(block) {
            let n = u - k;
            if n <= N && k <= K {
                let to_n = if n + block - 1 < N { n + block - 1 } else { N };
                let to_k = if k + block - 1 < K { k + block - 1 } else { K };
                dprintln!("({n}, {k})..=({to_n}, {to_k})");
                compute_block(dp.clone(), n, to_n, k, to_k);
            }
        }
        dprintln!();
    }
    dprintln!("{:?}", dp);

    let dp_reader = dp.read().unwrap();
    dp_reader.get(N, K)
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
