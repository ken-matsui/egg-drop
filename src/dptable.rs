use std::fmt::Display;
use std::hash::Hash;
use std::sync::Arc;

pub(crate) use lockfree::map::Map as LockFreeMap;

pub(crate) type DpTable<K, V> = Vec<Arc<LockFreeMap<K, V>>>;

/// NOTE: For V, please use cheap types that already implemented Copy
///
/// To avoid returning reference, we dereference the value and copy it.
#[inline]
pub(crate) fn get<K: Hash + Ord, V: Copy>(dp: &DpTable<K, V>, n: usize, k: K) -> V {
    *(dp[n].get(&k).unwrap().val())
}
#[inline]
pub(crate) fn insert<K: Hash + Ord, V: Copy>(dp: &DpTable<K, V>, n: usize, k: K, val: V) {
    dp[n].insert(k, val);
}

#[allow(non_snake_case)]
pub(crate) fn new(N: usize, K: usize) -> DpTable<usize, i32> {
    // K: width, N: height in the dp table to match dp[n][k] to W(n,k) in Wikipedia.
    let dp = vec![Arc::new(LockFreeMap::<usize, i32>::new()); N + 1];
    // dp[n][0] = 0 forall n s.t. n >= 0
    for n in 0..=N {
        dp[n].insert(0, 0);
    }
    // dp[1][k] = k forall k s.t. k >= 0
    for k in 0..=K {
        dp[1].insert(k, k as i32);
    }
    // items in (n < 2 || k < 1) are already calculated
    dp
}

#[allow(non_snake_case)]
pub(crate) fn print<V: Copy + Display>(dp: &DpTable<usize, V>, N: usize, K: usize) {
    for n in 0..=N {
        for k in 0..=K {
            print!("{} ", get(&dp, n, k));
        }
        println!();
    }
}
