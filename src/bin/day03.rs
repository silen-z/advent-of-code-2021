const INPUT: &'static str = include_str!("../../input/day03.txt");

const INPUT_SIZE: u32 = 12;

fn main() {
    let mut zeroes = [0u32; INPUT_SIZE as usize];
    let mut ones = [0u32; INPUT_SIZE as usize];

    for l in INPUT.lines() {
        let n = u32::from_str_radix(l, 2).unwrap();

        for i in 0..INPUT_SIZE {
            if n & 2u32.pow(i) > 0 {
                ones[i as usize] += 1;
            } else {
                zeroes[i as usize] += 1;
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..INPUT_SIZE {
        let bit = 2u32.pow(i);
        if ones[i as usize] > zeroes[i as usize] {
            gamma += bit;
        } else {
            epsilon += bit;
        }
    }

    println!("{:012b}", gamma);
    println!("{:012b}", epsilon);

    println!("{}", gamma * epsilon)
}
