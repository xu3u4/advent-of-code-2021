use std::collections::{HashSet, HashMap};

#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Instruction {
    direction: String,
    value: i16
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[derive(Debug)]
struct Dot {
    x: i16,
    y: i16
}

fn main() {
    // let lines = include_str!("./input.txt").split("\n");
    let mut parts = include_str!("./input.txt").split("\n\n");
    let dots = parts
        .nth(0)
        .unwrap()
        .split("\n").fold(HashSet::new(), |mut acc, line| {
            let mut splitted = line.split(",");
            let x = splitted.nth(0).unwrap().parse::<i16>().unwrap();
            let y = splitted.nth(0).unwrap().parse::<i16>().unwrap();
            acc.insert(Dot{x, y});
            acc
        });

    let folds = parts
        .nth(0)
        .unwrap()
        .split("\n")
        .fold(vec![], |mut acc, fold| {
            let mut instruction = fold.split(" ").last().unwrap().split("="); //.collect::<(String, i16)>();
            let direction = instruction.nth(0).unwrap().to_string();
            let value = instruction.nth(0).unwrap().parse::<i16>().unwrap();
            acc.push(Instruction{ direction, value});
            acc
        });

    let part1_ans = part1(&dots, &folds);
    println!("part1: {}", part1_ans);

    part2(&dots, &folds);

}

fn fold_paper (dots: &HashSet<Dot>, ins: &Instruction) -> HashSet<Dot>{
    dots
        .iter()
        .filter_map(|&dot| {
            let new_dot = if ins.direction == "x".to_string() {
                let difference = ins.value - dot.x;
                match difference.signum() {
                    -1 => Some(Dot{x: ins.value + difference, y: dot.y}),
                    1 => Some(dot),
                    _ => None
                }
            } else {
                let difference = ins.value - dot.y;
                match difference.signum() {
                    -1 => Some(Dot{x: dot.x, y: ins.value + difference}),
                    1 => Some(dot),
                    _ => None
                }
            };
            new_dot

        }).collect::<HashSet<Dot>>()
}

fn part1(dots: &HashSet<Dot>, folds: &Vec<Instruction>) -> usize {
    let ins = &folds[0];
    fold_paper(dots, ins).len()
}

fn part2(dots: &HashSet<Dot>, folds: &Vec<Instruction>) {
    let mut cloned_dots = dots.clone();
    folds.iter().for_each(|fold| {
        cloned_dots = fold_paper(&cloned_dots, fold);
    });

    let lines = folds.iter().fold(HashMap::new(), |mut acc, fold| {
        acc.entry(fold.direction.clone()).or_insert(vec![]).push(fold.value);
        acc
    });
    let x_min = lines.get("x").unwrap().iter().min().unwrap().abs() as usize;
    let y_min = lines.get("y").unwrap().iter().min().unwrap().abs() as usize;

    print!("part2:");

    for x in 0..=x_min {
        println!("");
        for y in 0..=y_min {
            if cloned_dots.contains(&Dot{x: x as i16, y: y_min as i16 - y as i16}) {
                print!("#");
            } else {
                print!(".");
            }
        }
    }
}