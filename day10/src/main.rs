use std::collections::HashMap;

fn main() {
    let lines = include_str!("./input.txt").split("\n").collect::<Vec<&str>>();



    let part1_ans = part1(&lines);
    let part2_ans = part2(&lines);
    println!("part 1: {}", part1_ans);

    println!("part 2: {:?}", part2_ans);

}

fn part1(lines: &Vec<&str>) -> i32 {
    let chunk_map: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);

    lines
        .iter()
        .map(|line| {
            let mut stack: Vec<char> = vec![];
            let mut finall: i32 = 0;
            for chunk in line.chars() {
                if chunk_map.contains_key(&chunk){
                    // is left
                    stack.push(chunk);
                } else if &chunk == chunk_map.get(stack.last().unwrap()).unwrap() {
                    // is matched right
                    stack.pop();
                } else {
                    // is not matched right
                    finall = match &chunk {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ =>  0,
                    };
                    break;
                }
            }
            finall
        }).sum::<i32>()
}

fn part2(lines: &Vec<&str>) -> i64 {
    let chunk_map: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);

    let mut scores = lines
        .iter()
        .map(|line| {
            let mut right_stack: Vec<char> = vec![];
            for chunk in line.chars() {
                if chunk_map.contains_key(&chunk){
                    // is left
                    right_stack.push(chunk_map.get(&chunk).unwrap().to_owned());
                } else if &chunk != right_stack.last().unwrap() {
                    // right is wrong
                    right_stack.clear();
                    break;
                } else {
                    // right is found
                    right_stack.pop();
                }
            }
            right_stack.reverse();
            right_stack.iter().fold(0, |acc: i64, chunk| {
                let point = match chunk {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ =>  0,
                };
                acc * 5 + point
            })
        }).filter(|score| score > &0)
        .collect::<Vec<i64>>();

    scores.sort();
    scores[scores.len()/2]

}
