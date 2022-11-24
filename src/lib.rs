#![feature(int_roundings)]

mod dp;
mod fast_dp;
mod par_dp;
mod rayon_par_dp;
mod rec;

pub use dp::dp;
pub use fast_dp::fast_dp;
pub use par_dp::par_dp;
pub use rayon_par_dp::rayon_par_dp;
pub use rec::rec;

#[inline]
fn constraints(n: i32, h: i32) {
    assert!(1 <= n, "constraints for n: number of eggs");
    assert!(1 <= h, "constraints for h: number of floors");
    // LeetCode Constraints
    // assert!(1 <= n && n <= 100, "constraints for n: number of eggs");
    // assert!(1 <= h && h <= 10000, "constraints for h: number of floors");
}

#[inline]
pub fn egg_drop(f: fn(i32, i32) -> i32, n: i32, h: i32) -> i32 {
    constraints(n, h);
    f(n, h)
}
