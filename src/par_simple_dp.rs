use std::sync::Arc;
use std::thread;

use crate::dptable::DpTable;
use crate::simple_dp::compute_block;

// ref: https://en.wikipedia.org/wiki/Dynamic_programming#Egg_dropping_puzzle
#[allow(non_snake_case)]
pub fn par_simple_dp(N: usize, K: usize) -> i32 {
    let dp = Arc::new(DpTable::new(N, K));

    let block = 100; // block*block sized block
    for u in (2..=(N + K)).step_by(block) {
        let mut threads = vec![];

        for k in (0..=u).step_by(block) {
            let dp_cloned = dp.clone();
            threads.push(thread::spawn(move || {
                let n = u - k;
                if n <= N && k <= K {
                    let to_n = if n + block - 1 < N { n + block - 1 } else { N };
                    let to_k = if k + block - 1 < K { k + block - 1 } else { K };
                    // println!("({n}, {k})..=({to_n}, {to_k})");
                    compute_block(dp_cloned, n, to_n, k, to_k);
                }
            }));
        }
        // println!();

        for thread in threads {
            // Wait for the thread to finish. Returns a result.
            let _ = thread.join();
        }
    }
    // println!("{:?}", dp);

    dp.get(N, K)
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
