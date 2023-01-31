use std::sync::{Arc, RwLock};
use threadpool::ThreadPool;

use debug_print::debug_println as dprintln;

use crate::dptable::DpTable;
use crate::simple_dp::compute_block;

#[allow(non_snake_case)]
pub fn par_simple_dp(N: usize, K: usize) -> i32 {
    let dp = Arc::new(RwLock::new(DpTable::new(N, K)));
    let block = 100; // block*block sized block

    let n_workers = if block > N || block > K {
        1
    } else if N / block >= K / block {
        N / block // 1000/100 = max 10 diagonals in the middle
    } else {
        K / block
    };
    let pool = ThreadPool::new(n_workers);

    for u in (2..=(N + K)).step_by(block) {
        for k in (0..=u).step_by(block) {
            let dp = dp.clone(); // Arc::clone

            pool.execute(move || {
                let n = u - k;
                if n <= N && k <= K {
                    let to_n = if n + block - 1 < N { n + block - 1 } else { N };
                    let to_k = if k + block - 1 < K { k + block - 1 } else { K };
                    dprintln!("({n}, {k})..=({to_n}, {to_k})");
                    compute_block(dp, n, to_n, k, to_k);
                }
            });
        }
        dprintln!();
        pool.join();
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
