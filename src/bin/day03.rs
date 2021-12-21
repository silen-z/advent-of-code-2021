const INPUT: &'static str = include_str!("../../input/day03.txt");

const INPUT_SIZE: u32 = 12;

fn main() {
    let numbers: Vec<u32> = INPUT
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect();

    let gamma = (0..INPUT_SIZE).fold(0, |gamma, i| gamma | most_common_bit(&numbers, i) << i);
    let epsilon = gamma ^ !(u32::MAX << INPUT_SIZE);

    println!("{}", gamma * epsilon);

    let mut oxygens = numbers.clone();
    for i in (0..INPUT_SIZE).rev() {
        if oxygens.len() == 1 {
            break;
        }
        let bit = most_common_bit(&oxygens, i);
        oxygens.retain(|n| (n >> i) & 1 == bit);
    }

    let mut co2s = numbers;
    for i in (0..INPUT_SIZE).rev() {
        if co2s.len() == 1 {
            break;
        }
        let bit = most_common_bit(&co2s, i) ^ 1;
        co2s.retain(|n| (n >> i) & 1 == bit);
    }

    match (oxygens.as_slice(), co2s.as_slice()) {
        ([oxygen], [co2]) => println!("{}", oxygen * co2),
        _ => panic!("expected to filter out single candidate instead got, oxygen: {}, co2: {}", oxygens.len(), co2s.len()),
    }
}

fn most_common_bit(numbers: &[u32], i: u32) -> u32 {
    let ones = numbers.iter().fold(0, |ones, n| ones + ((n >> i) & 1));
    (ones * 2 >= numbers.len() as u32) as u32
}
