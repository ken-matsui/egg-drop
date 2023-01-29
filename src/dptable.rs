use std::fmt::{Debug, Formatter};
use std::sync::Arc;

pub(crate) use lockfree::map::Map as LockFreeMap;

#[allow(non_snake_case)]
pub(crate) struct DpTable<V: Copy> {
    N: usize,
    K: usize,

    data: Vec<Arc<LockFreeMap<usize, V>>>, // (usize, usize)?
}

impl<V: Copy> DpTable<V> {
    /// NOTE: For V, please use cheap types that already implemented Copy
    ///
    /// To avoid returning reference, we dereference the value and copy it.
    #[inline]
    pub(crate) fn get(&self, n: usize, k: usize) -> V {
        *(self.data[n].get(&k).unwrap().val())
    }
    #[inline]
    pub(crate) fn insert(&self, n: usize, k: usize, val: V) {
        self.data[n].insert(k, val);
    }
}

impl DpTable<i32> {
    #[allow(non_snake_case)]
    pub(crate) fn new(N: usize, K: usize) -> Self {
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
        Self { N, K, data: dp }
    }
}

impl<V: Copy + Debug> Debug for DpTable<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for n in 0..=self.N {
            for k in 0..=self.K {
                write!(f, "{:?} ", self.get(n, k))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
