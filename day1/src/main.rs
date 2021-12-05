fn main() {
    part1();
    part2();
}

fn part1() {
    let depths = include_str!("../input.txt")
        .lines()
        .map(|depth| depth.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut count = 0;

    for (i, depth) in depths.iter().enumerate() {
        if i > 0 && depth > &depths[i - 1] {
            count = count + 1;
        }
    }

    println!("part 1 answer: {}", count);
}

fn part2() {
    let depths = include_str!("../input.txt")
        .lines()
        .map(|depth| depth.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut count = 0;

    for (i, depth) in depths.iter().enumerate() {
        if i + 3 < depths.len() && &depths[i + 3] > depth {
            count = count + 1;
        }
    }

    println!("part 2 answer: {}", count);
}
