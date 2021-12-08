fn main() {
    let lines = include_str!("../input.txt")
        .lines().collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}

const MAX: u32 = 11;

fn common_at<'a>(lines: &Vec<&'a str>, position: u32) -> (Vec<&'a str>, Vec<&'a str>) {
    let mut one: Vec<&str> = vec![];
    let mut zero: Vec<&str> = vec![];
    lines
        .iter()
        .for_each(|&line| {
            if line.chars().nth(position as usize) == Some('1') {
                one.push(line);
            } else {
                zero.push(line);
            }
        });
    (zero, one)
}

fn part1(lines: &Vec<&str>) {
    let most_bits = (0..=MAX)
    .map(|position| {
        let (zero, one) = common_at(lines, position);
        if one.len() > zero.len() { '1' } else { '0' }
    }).collect::<String>();

    let one = u32::from_str_radix(&most_bits, 2).unwrap();
    let zero = !one & 0xfff;

    println!("part 1 answer: {}", one*zero);
}

fn part2<'a>(lines: &Vec<&'a str>) {
    let mut most = lines.clone();
    let mut less = lines.clone();

    for n in 0..=MAX {
        let (zero, one) = common_at(&most, n);
        most = if one.len() >= zero.len() { one } else { zero };
        if most.len() == 1 {
            break;
        }
    }

    for n in 0..=MAX {
        let (zero, one) = common_at(&less, n);
        less = if zero.len() <= one.len() { zero } else { one };
        if less.len() == 1 {
            break;
        }
    }
    let result = u32::from_str_radix(most[0], 2).unwrap() * u32::from_str_radix(less[0], 2).unwrap();

    println!("part 2 answer: {:?}", result);
}
