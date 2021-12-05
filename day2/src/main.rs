fn main() {
    part1();
    part2();
}

struct Command {
    direction: String,
    distance: i32,
}

fn part1() {
    let commands = include_str!("../input.txt")
        .lines()
        .map(|command| {
            let details = command.split_whitespace().collect::<Vec<&str>>();
            return Command { direction: details[0].to_string(), distance: details[1].parse::<i32>().unwrap() };
        })
        .collect::<Vec<_>>();

    let mut x = 0;
    let mut y: i32 = 0;

    for command in commands.iter() {
        match &*command.direction {
            "forward" => x = x + command.distance,
            "down" => y = y + command.distance,
            "up" => y = y - command.distance,
            _ =>  println!("no match"),
        }
    }

    println!("part 1 answer: {}", x * y);
}

fn part2() {
    let commands = include_str!("../input.txt")
        .lines()
        .map(|command| {
            let details = command.split_whitespace().collect::<Vec<&str>>();
            return Command { direction: details[0].to_string(), distance: details[1].parse::<i32>().unwrap() };
        })
        .collect::<Vec<_>>();

    let mut x = 0;
    let mut aim = 0;
    let mut depth = 0;

    for command in commands.into_iter() {
        match &*command.direction {
            "forward" => {
                x = x + command.distance;
                depth = depth + aim * command.distance;
            },
            "down" => aim = aim + command.distance,
            "up" => aim = aim - command.distance,
            _ =>  println!("no match"),
        }
    }

    println!("part 2 answer: {}", x * depth);
}
