use threadpool::ThreadPool;

use debug_print::debug_println as dprintln;

use crate::dptable::DpTable;
use crate::simple_dp::compute_block;

#[allow(non_snake_case)]
// bsize*bsize sized block
pub fn par_simple_dp(N: usize, K: usize, bsize: usize) -> i32 {
    let mut dp = DpTable::new(N, K);
    let dp_p = dp.as_mut_ptr();

    let n_workers = if bsize > N || bsize > K {
        1
    } else if N / bsize >= K / bsize {
        N / bsize // 1000/100 = max 10 diagonal blocks in the middle
    } else {
        K / bsize
    };
    let pool = ThreadPool::new(n_workers);

    let mut u = 2;
    while u <= N + K {
        let mut k = 0;
        while k <= u {
            let dp_p = dp_p.clone();

            pool.execute(move || {
                let n = u - k;
                if n <= N && k <= K {
                    let to_n = if n + bsize - 1 < N { n + bsize - 1 } else { N };
                    let to_k = if k + bsize - 1 < K { k + bsize - 1 } else { K };
                    dprintln!("({n}, {k})..=({to_n}, {to_k})");
                    compute_block(dp_p, n, to_n, k, to_k);
                }
            });

            k += bsize; // step_by
        }
        dprintln!();
        pool.join();

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
    fn test_par_simple_dp() {
        assert_eq!(egg_drop(par_simple_dp, 8, 8, 2), 4); // useful for debugging
        assert_eq!(egg_drop(par_simple_dp, 2, 1, 2), 1);
        assert_eq!(egg_drop(par_simple_dp, 1, 2, 2), 2);
        assert_eq!(egg_drop(par_simple_dp, 2, 6, 2), 3);
        assert_eq!(egg_drop(par_simple_dp, 3, 14, 2), 4);
        assert_eq!(egg_drop(par_simple_dp, 4, 30, 2), 5);
        assert_eq!(egg_drop(par_simple_dp, 5, 62, 2), 6);
        assert_eq!(egg_drop(par_simple_dp, 6, 126, 2), 7);
        assert_eq!(egg_drop(par_simple_dp, 50, 500, 2), 9);
        assert_eq!(egg_drop(par_simple_dp, 10, 8, 2), 4);
        assert_eq!(egg_drop(par_simple_dp, 50, 500, 2), 9);
    }
}
