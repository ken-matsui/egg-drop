#![cfg(feature = "rayon")]

use std::sync::Arc;

use lockfree::map::Map as LockFreeMap;
use rayon::prelude::*;

#[allow(non_snake_case)]
pub fn rayon_par_fast_dp(N: usize, K: usize) -> i32 {
    if N == 1 || K == 0 || K == 1 {
        return K as i32;
    }

    let mut dp = vec![];
    for _ in 0..=K {
        dp.push(Arc::new(LockFreeMap::<usize, i32>::new()));
    }
    // Initialize LockFreeMap as 0_i32
    for n in 0..=N {
        dp[0].insert(n, 0);
    }

    let mut m = 0_usize;
    while dp[m].get(&N).unwrap().val() < &(K as i32) {
        m += 1;
        dp[m].insert(0, 0);
        (1..=N).into_par_iter().for_each(|k| {
            dp[m].insert(
                k,
                dp[m - 1].get(&(k - 1)).unwrap().val() + dp[m - 1].get(&k).unwrap().val() + 1,
            );
        });
    }

    m as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::egg_drop;

    #[test]
    fn test_rayon_par_dp() {
        assert_eq!(egg_drop(rayon_par_fast_dp, 2, 1), 1);
        assert_eq!(egg_drop(rayon_par_fast_dp, 1, 2), 2);
        assert_eq!(egg_drop(rayon_par_fast_dp, 2, 7), 4);
        assert_eq!(egg_drop(rayon_par_fast_dp, 3, 14), 4);
        assert_eq!(egg_drop(rayon_par_fast_dp, 4, 30), 5);
        assert_eq!(egg_drop(rayon_par_fast_dp, 5, 62), 6);
        assert_eq!(egg_drop(rayon_par_fast_dp, 6, 126), 7);
        assert_eq!(egg_drop(rayon_par_fast_dp, 50, 500), 9);
    }
}
