use std::cmp::{max, min};

// ref: https://en.wikipedia.org/wiki/Dynamic_programming#Egg_dropping_puzzle
pub fn simple_dp(n: i32, k: i32) -> i32 {
    #[allow(non_snake_case)]
    let N = n as usize;
    #[allow(non_snake_case)]
    let K = k as usize;

    let mut numdrops = vec![vec![0_i32; K + 1]; N + 1];

    for i in 0..=k {
        numdrops[1][i as usize] = i;
    }

    for i in 2..=N {
        for j in 1..=K {
            let mut minval = i32::MAX;
            for x in 1..=j {
                minval = min(minval, max(numdrops[i][j - x], numdrops[i - 1][x - 1]));
            }
            numdrops[i][j] = 1 + minval;
        }
    }
    numdrops[N][K]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::egg_drop;

    #[test]
    fn test_dp() {
        assert_eq!(egg_drop(simple_dp, 2, 1), 1);
        assert_eq!(egg_drop(simple_dp, 1, 2), 2);
        assert_eq!(egg_drop(simple_dp, 2, 6), 3);
        assert_eq!(egg_drop(simple_dp, 3, 14), 4);
        assert_eq!(egg_drop(simple_dp, 4, 30), 5);
        assert_eq!(egg_drop(simple_dp, 5, 62), 6);
        assert_eq!(egg_drop(simple_dp, 6, 126), 7);
        assert_eq!(egg_drop(simple_dp, 50, 500), 9);
    }
}
