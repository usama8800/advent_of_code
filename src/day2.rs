use color_eyre::eyre::Result;
use std::fs;

enum Direction {
    Up,
    Down,
    Forward,
}

struct Command {
    direction: Direction,
    magnitute: u32,
}

fn get_input() -> Result<Vec<Command>> {
    let contents = fs::read_to_string("inputs/day2.txt").expect("");
    let splits = contents.split_terminator('\n');

    Ok(splits
        .filter_map(|v| {
            let splits: Vec<&str> = v.split(' ').collect();
            let num: u32 = (*splits.get(1)?).parse().ok()?;
            let direction = match *splits.get(0)? {
                "forward" => Direction::Forward,
                "up" => Direction::Up,
                "down" => Direction::Down,
                _ => unreachable!("direction {}", *splits.get(0)?),
            };

            Some(Command {
                direction,
                magnitute: num,
            })
        })
        .collect())
}

fn solve_p1() -> Result<()> {
    let commands = get_input()?;

    let mut horizontal = 0;
    let mut depth = 0;
    for command in commands {
        match command {
            Command {
                direction: Direction::Forward,
                magnitute,
            } => horizontal += magnitute,
            Command {
                direction: Direction::Down,
                magnitute,
            } => depth += magnitute,
            Command {
                direction: Direction::Up,
                magnitute,
            } => depth -= magnitute,
            _ => unreachable!(),
        }
    }
    dbg!(horizontal, depth);
    dbg!(horizontal * depth);

    Ok(())
}

pub fn solve_p2() -> Result<()> {
    let commands = get_input()?;

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in commands {
        match command {
            Command {
                direction: Direction::Forward,
                magnitute,
            } => {
                horizontal += magnitute;
                depth += aim * magnitute;
            }
            Command {
                direction: Direction::Down,
                magnitute,
            } => aim += magnitute,
            Command {
                direction: Direction::Up,
                magnitute,
            } => aim -= magnitute,
            _ => unreachable!(),
        }
    }
    dbg!(horizontal, depth);
    dbg!(horizontal * depth);

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    solve_p2();

    Ok(())
}
