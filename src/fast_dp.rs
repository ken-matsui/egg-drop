// ref: https://en.wikipedia.org/wiki/Dynamic_programming#Faster_DP_solution_using_a_different_parametrization
pub fn fast_dp(n: i32, h: i32) -> i32 {
    #[allow(non_snake_case)]
    let N = n as usize;
    #[allow(non_snake_case)]
    let H = h as usize;

    let mut memo = vec![vec![0_i32; N + 1]; H + 1];
    let mut m = 0_usize;
    while memo[m][N] < h {
        m += 1;
        for k in 1..=N {
            memo[m][k] = memo[m - 1][k - 1] + memo[m - 1][k] + 1;
        }
    }
    m as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::egg_drop;

    #[test]
    fn test_fast_dp() {
        assert_eq!(egg_drop(fast_dp, 2, 1), 1);
        assert_eq!(egg_drop(fast_dp, 1, 2), 2);
        assert_eq!(egg_drop(fast_dp, 2, 7), 4);
        assert_eq!(egg_drop(fast_dp, 3, 14), 4);
        assert_eq!(egg_drop(fast_dp, 4, 30), 5);
        assert_eq!(egg_drop(fast_dp, 5, 62), 6);
        assert_eq!(egg_drop(fast_dp, 6, 126), 7);
        assert_eq!(egg_drop(fast_dp, 50, 500), 9);
    }
}
