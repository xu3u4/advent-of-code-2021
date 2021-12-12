#![allow(unused)]
fn main() {
    let  numbers: Vec<u8> = include_str!("../input.txt")
        .split(',')
        .map(|num| num.parse::<u8>().unwrap())
        .collect();

    println!("part1 answer: {:?}", laternfish(& numbers, 80));
    println!("part2 answer: {:?}", laternfish(& numbers, 256));
}

fn laternfish(numbers: & Vec<u8>, days: i32) -> u64 {
    let mut boxes: Vec<u64> = vec![0; 9];
    for &number in numbers {
        boxes[(number as usize)] += 1;
    }

    for i in 1..=days {
        let new_fish = boxes[0];
        (0..8).for_each(|i| boxes[i] = boxes[i + 1]);
        boxes[6] += new_fish;
        boxes[8] = new_fish;
    }

    boxes
    .iter()
    .sum()
}
