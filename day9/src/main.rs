use std::collections::HashSet;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let lines = include_str!("./input.txt").split("\n");
    let geo = lines
    .map(|line| {
        line
            .chars()
            .map(|char| {
                char.to_digit(10).unwrap()
            })
            .collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();

    let (part1_ans, low_points) = part1(geo.clone());
    // println!("low_points: {:?}", &low_points);
    let part2_ans = part2(geo, low_points);
    println!("part 1: {}", part1_ans);

    println!("part 2: {:?}", part2_ans);
}

fn get_or_skip(geo: &Vec<Vec<u32>>, x: Option<usize>, y: Option<usize>) -> Option<Point> {
    match (x, y) {
        (None, _) => None,
        (_, None) => None,
        (Some(x), Some(y)) => {
            geo
            .get(x)
            .and_then(|line| line.get(y).copied())
            .map(|_| Point{x, y})
        }
    }
}

fn get_neighbors(geo: &Vec<Vec<u32>>, x: usize, y: usize) -> [Option<Point>; 4] {
    [
        get_or_skip(&geo, x.checked_sub(1), Some(y)),
        get_or_skip(&geo, Some(x + 1), Some(y)),
        get_or_skip(&geo, Some(x), y.checked_sub(1)),
        get_or_skip(&geo, Some(x), Some(y + 1)),
    ]
}

fn part1(geo: Vec<Vec<u32>>) -> (u32, Vec<Point>) {
    let mut low_points_sum:u32 = 0;
    let mut low_points: Vec<Point> = Vec::new();
    geo
        .clone()
        .iter()
        .enumerate()
        .for_each(|(i, line)| {
            line
                .iter()
                .enumerate()
                .for_each(|(j, num)| {
                    let is_cave = get_neighbors(&geo, i, j)
                    .iter()
                    .all(|neighbor| {
                        match neighbor {
                            None => true,
                            Some(point) => {
                                geo[point.x][point.y] > *num
                            }
                        }
                        // yayoc finds the min from neighbors and compare to it.
                    });

                    if is_cave {
                        low_points.push(Point{x: i, y: j });
                        low_points_sum += *num + 1;
                    }
                })
        });
    (low_points_sum, low_points)
}

fn get_low_neighbors(target: &Point, geo: &Vec<Vec<u32>>, visted_points: HashSet<Point>) -> HashSet<Point> {
    let new_neighbors: HashSet<Point> = get_neighbors(&geo, target.x, target.y)
        .to_vec()
        .iter()
        .filter_map(|&neighbor| {
            neighbor.filter(|n| !visted_points.contains(&n))
        })
        .collect::<HashSet<Point>>();

    new_neighbors.iter().fold(visted_points, |mut low_neighbor, point| {
        if geo[point.x][point.y] != 9 {
            low_neighbor.insert(*point);
            return get_low_neighbors(point, geo, low_neighbor)
        }

        low_neighbor
    })
}

fn part2(geo: Vec<Vec<u32>>, low_points: Vec<Point>) -> usize {
    let mut basin_groups = low_points
    .iter()
    .map(|target| {
        let mut visited = HashSet::new();
        visited.insert(*target);
        let temp = get_low_neighbors(target, &geo, visited);

        temp.len()
    }).collect::<Vec<usize>>();

    basin_groups.sort();
    let basin_len = basin_groups.len();
    basin_groups[basin_len - 1] * basin_groups[basin_len - 2] * basin_groups[basin_len - 3]
}
