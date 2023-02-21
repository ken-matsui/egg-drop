use std::fmt::{Debug, Formatter};

// Can send to threads + Mutable + Ptr
#[derive(Clone)]
pub(crate) struct DpTablePtr<V>(pub(crate) *mut Vec<V>);
unsafe impl<V> Sync for DpTablePtr<V> {}
unsafe impl<V> Send for DpTablePtr<V> {}

impl<V: Copy> DpTablePtr<V> {
    #[inline]
    pub(crate) unsafe fn get(&self, n: usize, k: usize) -> V {
        (*self.0.add(n))[k]
    }
    #[inline]
    pub(crate) unsafe fn insert(&self, n: usize, k: usize, val: V) {
        (*self.0.add(n))[k] = val;
    }
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn as_const_ptr(&self) -> *const Vec<V> {
        self.0
    }
}

#[allow(non_snake_case)]
pub(crate) struct DpTable<V: Copy> {
    N: usize,
    K: usize,

    data: Vec<Vec<V>>,
}

impl<V: Copy> DpTable<V> {
    #[inline]
    pub(crate) fn get(&self, n: usize, k: usize) -> V {
        self.data[n][k]
    }
    #[inline]
    pub(crate) fn insert(&mut self, n: usize, k: usize, val: V) {
        self.data[n][k] = val;
    }

    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> DpTablePtr<V> {
        DpTablePtr(self.data.as_mut_ptr())
    }
}

impl DpTable<i32> {
    #[allow(non_snake_case)]
    pub(crate) fn new(N: usize, K: usize) -> Self {
        // K: width, N: height in the dp table to match dp[n][k] to W(n,k) in Wikipedia.
        let mut dp = Self {
            N,
            K,
            data: vec![vec![0_i32; K + 1]; N + 1],
        };
        // dp[n][0] = 0 forall n s.t. n >= 0
        // inv: dp[1][k] = k forall k s.t. k >= 0
        for k in 0..=K {
            dp.insert(1, k, k as i32);
        }
        // items in (n < 2 || k < 1) are already calculated
        dp
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
