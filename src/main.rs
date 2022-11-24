#![feature(int_roundings)]

fn main() {
    #[allow(non_snake_case)]
    let N = 50_usize;
    let num_threads = 10_usize;
    let steps = N.div_ceil(num_threads);

    for from in (1..=N).step_by(steps) {
        let to = if from + steps < N {
            from + steps
        } else {
            N + 1
        };

        println!("{}..{}", from, to);
    }
}
