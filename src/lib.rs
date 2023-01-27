#![feature(int_roundings)]

mod fast_dp;
mod par_fast_dp;
mod par_simple_dp;
mod rayon_par_fast_dp;
mod rec;
mod simple_loop_dp;

pub use fast_dp::fast_dp;
pub use par_fast_dp::par_fast_dp;
pub use rayon_par_fast_dp::rayon_par_fast_dp;
pub use rec::rec;
pub use simple_loop_dp::simple_loop_dp;

// LeetCode Constraints
// assert!(1 <= n && n <= 100, "constraints for n: number of eggs");
// assert!(1 <= h && h <= 10000, "constraints for h: number of floors");

#[inline]
fn precondition(n: usize, k: usize) {
    assert!(1 <= n, "constraints for n: number of eggs");
    assert!(1 <= k, "constraints for h: number of floors");
}

#[inline]
pub fn egg_drop(f: fn(usize, usize) -> i32, n: usize, k: usize) -> i32 {
    precondition(n, k);
    f(n, k)
}
