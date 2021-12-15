use std::collections::HashMap;

fn main() {
    let number_hash = include_str!("./input.txt")
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .fold(HashMap::new(), |mut acc, number| {
            *acc.entry(number).or_insert(0) += 1;
            acc
        });
    let part1_ans = part1(&number_hash);
    let part2_ans = part2(&number_hash);
    println!("part1: {}", part1_ans);
    println!("part2: {}", part2_ans);
}

fn part1(number_hash: & HashMap<i32, i32>) -> i64 {
    let min = *number_hash.keys().min().unwrap();
    let max = *number_hash.keys().max().unwrap();
    let mut min_total: i64 = 0;

    for n in min..max {
        let mut total: i64 = 0;
        number_hash.iter().for_each(|(k, v)| {
            total += ((n - *k).abs() * *v ) as i64;
        });
        if min_total == 0 || total < min_total {
            min_total = total;
        }
    }

    min_total
}

fn count_steps(target: usize, memory: &mut Vec<i64>) -> i64 {
    if memory.get(target) != None {
        memory[target]
    } else {
        let prev = count_steps(target - 1, memory);
        memory.push(prev + target as i64);
        prev + target as i64
    }
}

fn part2(number_hash: & HashMap<i32, i32>)-> i64 {
    let min = *number_hash.keys().min().unwrap();
    let max = *number_hash.keys().max().unwrap();
    let mut min_total: i64 = 0;
    let mut steps: Vec<i64> = vec![0];

    for n in min..max {
        let mut total: i64 = 0;
        number_hash.iter().for_each(|(k, v)| {
            total +=
                count_steps(
                    (n - *k).abs() as usize,
                    &mut steps
                ) * *v as i64;
        });
        if min_total == 0 || total < min_total {
            min_total = total;
        }
    }

    min_total
}