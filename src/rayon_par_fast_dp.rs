#![cfg(feature = "rayon")]

use std::sync::Arc;

use lockfree::map::Map as LockFreeMap;
use rayon::prelude::*;

pub fn rayon_par_fast_dp(n: i32, h: i32) -> i32 {
    if n == 1 || h == 0 || h == 1 {
        return h;
    }

    #[allow(non_snake_case)]
    let N = n as usize;
    #[allow(non_snake_case)]
    let H = h as usize;

    let mut memo = vec![];
    for _ in 0..=H {
        memo.push(Arc::new(LockFreeMap::<usize, i32>::new()));
    }
    // Initialize LockFreeMap as 0_i32
    for i in 0..=N {
        memo[0].insert(i, 0);
    }

    let mut m = 0_usize;
    while memo[m].get(&N).unwrap().val() < &h {
        m += 1;
        memo[m].insert(0, 0);
        (1..=N).into_par_iter().for_each(|k| {
            memo[m].insert(
                k,
                memo[m - 1].get(&(k - 1)).unwrap().val() + memo[m - 1].get(&k).unwrap().val() + 1,
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
