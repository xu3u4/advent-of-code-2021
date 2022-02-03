use std::collections::HashMap;

fn main() {
    let lines = include_str!("./input.txt").split("\n");
    let map = lines.fold(HashMap::new(), |mut acc, line| {
        let mut splitted = line.split("-");
        let start = splitted.nth(0).unwrap();
        let end = splitted.nth(0).unwrap();
        acc.entry(start.clone()).or_insert(vec![]).push(end.clone());
        acc.entry(end).or_insert(vec![]).push(start);
        acc
    });
    println!("{:?}", map);
    let part1_ans = part1(map);
    println!("part1: {}", part1_ans);
}

fn is_lower_case(str: &str) -> bool {
   str.to_lowercase() == str
}

fn get_connection(map: &HashMap<&str, Vec<&str>>, entry: &str, paths: &Vec<String>, completed: &mut Vec<Vec<String>>) {
    let start = map.get(entry).unwrap();

    for &point in start.iter() {
        let temp = point.to_owned();
        let mut new_path = paths.clone();

        if paths.contains(&temp) && is_lower_case(point) {
            continue;
        }

        new_path.push(temp);

        if point != "end" {
            get_connection(&map, point, &new_path, completed);
        } else {
            completed.push(new_path);
        }
    }
}

fn part1(map: HashMap<&str, Vec<&str>>) -> usize {
    let paths: Vec<String> = vec!["start".to_string()];
    let mut complete: Vec<Vec<String>> = vec![];

    get_connection(&map, "start", &paths, &mut complete);

    complete.len()
}
