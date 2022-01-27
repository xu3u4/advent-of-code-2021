use std::collections::HashMap;

struct S {
    patterns: Vec<String>,
    values: Vec<String>
}
fn main() {
    let lines = include_str!("./input.txt")
        .lines()
        .map(|line| {
            let splitted = line.split(" | ");
            let patterns = splitted.clone().nth(0)
                .unwrap()
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            let values = splitted.clone().nth(1)
                .unwrap()
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            S { patterns, values }
        })
        .collect::<Vec<_>>();

    let part1_ans = part1(&lines);
    let part2_ans = part2(&lines);
    println!("part1: {}", part1_ans);

}



fn part1(outputs: &Vec<S>) -> usize {
    let segments: HashMap<i8, usize> = HashMap::from([
        (0, 6),
        (1, 2),
        (2, 5),
        (3, 5),
        (4, 4),
        (5, 5),
        (6, 6),
        (7, 3),
        (8, 7),
        (9, 6),
    ]);

    outputs
        .iter()
        .fold(0, |acc, line| {
            line.values.iter().fold(0, |mut line_acc, number| {
                let l = &number.len();
                if l == segments.get(&1).unwrap()
                    || l == segments.get(&4).unwrap()
                    || l == segments.get(&7).unwrap()
                    || l == segments.get(&8).unwrap() {
                        line_acc += 1;
                    }
                line_acc
            }) + acc
        })
}

middle_upper = 7 - 1
middle_center = 8 - 0
middle_bottom = 8 - 4 - middle_upper - left_bottom
left_upper = 5 + right_upper - 3
left_bottom = 6 - 5
right_upper = 8 - 6
right_bottom = 3 + left_bottom - 2
struct Detail {
    count: usize,
    segment: Option<string>,
}

fn part2(outputs: &Vec<S>) {
    let mut segments: HashMap<i8, Detail> = HashMap::from([
        (0, Detail{ count: 6, }),
        (1, Detail{ count: 2, }),
        (2, Detail{ count: 5, }),
        (3, Detail{ count: 3, }),
        (4, Detail{ count: 4, }),
        (5, Detail{ count: 5, }),
        (6, Detail{ count: 6, }),
        (7, Detail{ count: 3, }),
        (8, Detail{ count: 7, }),
        (9, Detail{ count: 6, }),
    ]);
}
