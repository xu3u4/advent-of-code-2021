use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Coor {
    x: usize,
    y: usize,
}

type Board = HashMap<u16, Coor>;

type MarkedBoard = Vec<Vec<i16>>;

fn main() {
    let mut strings = include_str!("../input.txt").split("\n\n");
    let draws: Vec<u16> = strings.nth(0).unwrap().split(",").map(|lot| lot.parse::<u16>().unwrap()).collect();
    let boards = strings.map(|board_string| {
        board_string
            .split("\n")
            .enumerate()
            .fold(HashMap::new(), |mut map, (i, line)| {
                if line.is_empty() {
                    map
                } else {
                    line
                        .split_ascii_whitespace()
                        .enumerate()
                        .for_each(|(j, number)| {
                            let u16_number = number.parse::<u16>().unwrap();
                            map.insert(u16_number, Coor{ x: i, y: j});
                        });
                    map
                }
            })
    }).collect();


    let part1_ans = part1(&draws, &boards);
    let part2_ans = part2(&draws, &boards);

    println!("part 1: {}", part1_ans);
    println!("part 2: {}", part2_ans);
}

fn check_is_bingo(content: &MarkedBoard, x: usize, y: usize) -> bool {
    if content[x].iter().all(|&marked| marked >= 0) {
        true
    } else if content.iter().all(|row| row[y] >= 0) {
        true
    } else {
        false
    }
}

fn sum_board_values(board: &MarkedBoard) -> u16 {
    board.iter().fold(0, |acc, row| {
        let sum1 = row
            .iter()
            .filter(|n| n >= &&0)
            .fold(0, |row_acc, number| row_acc + number);
        acc + sum1 as u16
    })
}

fn part1(draws: &Vec<u16>, boards: &Vec<Board>) -> u16 {
    let mut marked_boards: Vec<MarkedBoard> = vec![vec![vec![-1; 5]; 5]; boards.len()];

    for draw in draws {
        for (nth, board) in boards.into_iter().enumerate() {
            if board.get(&draw).is_some() {
                let Coor{x, y} = board[&draw];
                marked_boards[nth][x][y] = *draw as i16;

                let bingo = check_is_bingo(&marked_boards[nth], x, y);
                if bingo {
                    let total = board.keys().sum::<u16>() - sum_board_values(&marked_boards[nth]);
                    return draw * total
                }
            }
        }
    }
    return 0
}

fn part2(draws: &Vec<u16>, boards: &Vec<Board>) -> u16 {
    let mut marked_boards: Vec<MarkedBoard> = vec![vec![vec![-1; 5]; 5]; boards.len()];
    let mut finished_boards: HashSet<usize> = HashSet::new();

    for draw in draws {
        for (nth, board) in boards.into_iter().enumerate() {
            if board.get(&draw).is_some() && !finished_boards.contains(&nth) {
                let Coor{x, y} = board[&draw];
                marked_boards[nth][x][y] = *draw as i16;

                let bingo = check_is_bingo(&marked_boards[nth], x, y);
                if bingo { finished_boards.insert(nth); }

                if finished_boards.len() == boards.len() {
                    let total = board.keys().sum::<u16>() - sum_board_values(&marked_boards[nth]);
                    return draw * total
                }
            }
        }
    }
    return 0
}

