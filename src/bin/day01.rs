const INPUT: &'static str = include_str!("../../input/day01.txt");

fn main() {
    let inputs: Vec<i32> =  INPUT.lines().filter_map(|l| l.parse().ok()).collect();

    println!("{}", count_increases(&inputs));

    let sums: Vec<i32> = inputs.windows(3).map(|w| w.iter().sum()).collect();
    println!("{}", count_increases(&sums));
}

fn count_increases<N: PartialOrd>(measurements: &[N]) -> usize {
    measurements.windows(2).filter(|w| w[1] > w[0]).count()
}