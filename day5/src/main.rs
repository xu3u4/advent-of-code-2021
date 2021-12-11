use std::collections::HashMap;

fn main() {
    let strings = include_str!("../input.txt")
        .split("\n")
        .map(|line| {
            line
                .split(" -> ")
                .map(|point_str| {
                    let points = point_str.split(",").collect::<Vec<_>>();
                    Point{
                        x: points[0].parse::<i16>().unwrap(),
                        y: points[1].parse::<i16>().unwrap()
                    }
                }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    let part1_ans = part1(&strings);
    println!("part1 answer: {}", part1_ans);
    let part2_ans = part2(&strings);
    println!("part2 answer: {}", part2_ans);
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i16,
    y: i16,
}

fn part1(lines: &Vec<Vec<Point>>) -> usize {
    let marked_points = create_point_count_map(lines, false);

    find_intersections(marked_points)
}

fn find_intersections(point_count_map: HashMap<Point, i16>) -> usize {
    point_count_map
    .values()
    .filter(|&&count| count > 1)
    .count()
}

fn create_point_count_map(lines: &Vec<Vec<Point>>, include_diagonal: bool) -> HashMap<Point, i16> {
    lines
    .iter()
    .fold(HashMap::new(), |mut acc, line| {
        let x_increment: i16 = (line[1].x - line[0].x).signum();
        let y_increment: i16 = (line[1].y - line[0].y).signum();

        if include_diagonal || x_increment == 0 || y_increment == 0 {
            let mut x = line[0].x;
            let mut y = line[0].y;
            while x != line[1].x + x_increment || y != line[1].y + y_increment {
                *acc.entry(Point {x, y}).or_insert(0) += 1;
                x += x_increment;
                y += y_increment;
            }
        }
        acc
    })
}

fn part2(lines: &Vec<Vec<Point>>) -> usize {
    let marked_points = create_point_count_map(lines, true);

    find_intersections(marked_points)
}