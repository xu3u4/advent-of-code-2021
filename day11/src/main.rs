use std::collections::HashSet;

type Octpuses = Vec<Vec<u32>>;
fn main() {
    let lines = include_str!("./input.txt").split("\n");
    let octopuses = lines
    .map(|line| {
        line
            .chars()
            .map(|char| {
                char.to_digit(10).unwrap()
            })
            .collect::<Vec<u32>>()
    }).collect::<Octpuses>();

    let part1_ans = part1(octopuses.clone());
    println!("part 1: {}", part1_ans);

    let part2_ans = part2(octopuses.clone());
    println!("part 2: {:?}", part2_ans);
}

fn increase_energy(updated_octoposes: &mut Octpuses, (i, j): (usize, usize), flashes: &mut i32) {
    let i_min = i.checked_sub(1).unwrap_or(0);
    let j_min = j.checked_sub(1).unwrap_or(0);
    let i_max = if i + 1 >= updated_octoposes.len() { i } else { i + 1 };
    let j_max = if j + 1 >= updated_octoposes.len() { j } else { j + 1 };

    for ii in i_min..=i_max {
        for jj in j_min..=j_max {
            if ii == i && jj == j {
                continue;
            }
            let new_value = if updated_octoposes[ii][jj] == 0 { 0 } else { updated_octoposes[ii][jj] + 1};
            updated_octoposes[ii][jj] = new_value;


            if new_value > 9 {
                updated_octoposes[ii][jj] = 0;
                *flashes += 1;
                increase_energy(updated_octoposes, (ii, jj), flashes);

            }
        }
    }
}

fn part1(octopuses: Octpuses) -> i32 {
    let mut flashes = 0;
    let mut updated_octoposes = octopuses.clone();
    let length = octopuses.len();

    let mut step = 1;

    while step <= 100 {
        let mut reset: HashSet<(usize, usize)> = HashSet::new();

        for i in 0..length {
            for j in 0..length {
                let new_value = updated_octoposes[i][j] + 1;
                updated_octoposes[i][j] = new_value;
                if new_value > 9 {
                    flashes += 1;
                    updated_octoposes[i][j] = 0;
                    reset.insert((i, j));
                }
            }
        }

        reset.iter().for_each(|(i, j)| {
            increase_energy(&mut updated_octoposes, (*i, *j), &mut flashes);
        });

        step += 1;
    }

    flashes
}

fn part2(octopuses: Octpuses) -> i32{
    let mut flashes: i32 = 0;
    let mut updated_octoposes = octopuses.clone();
    let length = octopuses.len();

    let mut step = 0;

    while flashes < (length * length) as i32 {
        let mut reset: HashSet<(usize, usize)> = HashSet::new();
        flashes = 0;
        for i in 0..length {
            for j in 0..length {
                let new_value = updated_octoposes[i][j] + 1;
                updated_octoposes[i][j] = new_value;
                if new_value > 9 {
                    flashes += 1;
                    updated_octoposes[i][j] = 0;
                    reset.insert((i, j));
                }
            }
        }

        reset.iter().for_each(|(i, j)| {
            increase_energy(&mut updated_octoposes, (*i, *j), &mut flashes);
        });

        step += 1;
    }

    step
}