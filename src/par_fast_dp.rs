use crate::fast_dp::fast_dp;

use std::sync::Arc;
use std::thread::{self, available_parallelism};

use lockfree::map::Map as LockFreeMap;

#[allow(non_snake_case)]
pub fn par_fast_dp(N: usize, K: usize) -> i32 {
    if N == 1 || K == 0 || K == 1 {
        return K as i32;
    }

    let mut memo = vec![];
    for _ in 0..=K {
        memo.push(Arc::new(LockFreeMap::<usize, i32>::new()));
    }
    // Initialize LockFreeMap as 0_i32
    for n in 0..=N {
        memo[0].insert(n, 0);
    }

    let num_threads = available_parallelism().unwrap().get();
    assert!(num_threads >= 1_usize);
    // Each thread must have 2 or more calculation units.
    if N + 1 < num_threads * 2 {
        return fast_dp(N, K);
    }

    // Narrow down number of chunks into <=num_threads
    let steps = N.div_ceil(num_threads);
    let mut m = 0_usize;
    while memo[m].get(&N).unwrap().val() < &(K as i32) {
        m += 1;
        memo[m].insert(0, 0);

        let mut threads = vec![];
        for from in (1..=N).step_by(steps) {
            let memo_m = memo[m].clone();
            let memo_m_1 = memo[m - 1].clone();

            threads.push(thread::spawn(move || {
                let to = if from + steps < N {
                    from + steps
                } else {
                    N + 1
                };

                for k in from..to {
                    memo_m.insert(
                        k,
                        memo_m_1.get(&(k - 1)).unwrap().val() + memo_m_1.get(&k).unwrap().val() + 1,
                    );
                }
            }));
        }

        for thread in threads {
            // Wait for the thread to finish. Returns a result.
            let _ = thread.join();
        }
    }
    m as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::egg_drop;

    #[test]
    fn test_par_dp() {
        assert_eq!(egg_drop(par_fast_dp, 2, 1), 1);
        assert_eq!(egg_drop(par_fast_dp, 1, 2), 2);
        assert_eq!(egg_drop(par_fast_dp, 2, 7), 4);
        assert_eq!(egg_drop(par_fast_dp, 3, 14), 4);
        assert_eq!(egg_drop(par_fast_dp, 4, 30), 5);
        assert_eq!(egg_drop(par_fast_dp, 5, 62), 6);
        assert_eq!(egg_drop(par_fast_dp, 6, 126), 7);
        assert_eq!(egg_drop(par_fast_dp, 50, 500), 9);
    }
}
