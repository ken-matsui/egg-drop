#![feature(int_roundings)]

#[allow(dead_code)]
fn div_rounding() {
    #[allow(non_snake_case)]
    let N = 99_usize;
    let num_threads = 3_usize;
    let steps = N.div_ceil(num_threads);

    for from in (1..=N).step_by(steps) {
        let to = if from + steps < N {
            from + steps
        } else {
            N + 1
        };

        println!("{from}..{to}");
    }
}

#[allow(dead_code)]
fn step_by() {
    #[allow(non_snake_case)]
    let N = 99_usize;
    let n = 8_usize;

    for from in (1..=N).step_by(n) {
        let to = if from + n < N { from + n } else { N };
        println!("{from}..={to}");
    }
}

#[allow(dead_code)]
fn emulate_step_by() {
    #[allow(non_snake_case)]
    let N = 99_usize;
    let n = 8_usize;

    let mut from = 1;
    while from <= N {
        let to = if from + n < N { from + n } else { N };
        println!("{from}..={to}");

        from += n; // step_by
    }
}

#[allow(dead_code)]
fn diagonal_loop() {
    #[allow(non_snake_case)]
    let WIDTH: usize = 7;
    #[allow(non_snake_case)]
    let HEIGHT: usize = 3;
    let mut ch = b'A';

    let mut array: Vec<Vec<char>> = vec![vec![ch as char; WIDTH]; HEIGHT];
    #[allow(clippy::needless_range_loop)]
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            array[i][j] = ch as char;
            ch += 1;
        }
    }

    #[allow(clippy::needless_range_loop)]
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            print!("{} ", array[i][j]);
        }
        println!();
    }

    println!();

    for k in 0..=(WIDTH + HEIGHT - 2) {
        for j in 0..=k {
            let i = k - j;
            if i < HEIGHT && j < WIDTH {
                print!("{} ", array[i][j]);
            }
        }
        println!();
    }
}

fn main() {
    // div_rounding();
    // step_by();
    emulate_step_by();
    // diagonal_loop();
}
