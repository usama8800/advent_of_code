use color_eyre::eyre::{eyre, Result};
use num::integer::Roots;
use regex::Regex;
use std::{
    fs,
    ops::{Add, AddAssign},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point(i16, i16);

impl Point {
    fn distance_to_point(&self, other: Self) -> u16 {
        ((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2))
            .sqrt()
            .try_into()
            .unwrap()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Debug)]
struct Rect {
    start: Point,
    end: Point,
}

impl Rect {
    fn center(&self) -> Point {
        Point(
            (self.start.0 + self.end.0) / 2,
            (self.start.1 + self.end.1) / 2,
        )
    }

    fn width(&self) -> u16 {
        self.start.0.abs_diff(self.end.0) + 1
    }

    fn height(&self) -> u16 {
        self.start.1.abs_diff(self.end.1) + 1
    }

    fn contains(&self, point: Point) -> bool {
        point.0 >= self.start.0
            && point.0 <= self.end.0
            && point.1 >= self.start.1
            && point.1 <= self.end.1
    }
}

fn get_input() -> Result<Rect> {
    let contents = fs::read_to_string("inputs/day17.txt").expect("");
    let mut lines = contents.split('\n').filter_map(|v| {
        if v.len() == 0 || v.chars().nth(0) == Some('#') {
            None
        } else {
            Some(v)
        }
    });

    let re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let captures = re.captures(lines.next().unwrap()).unwrap();

    Ok(Rect {
        start: Point(captures[1].parse().unwrap(), captures[3].parse().unwrap()),
        end: Point(captures[2].parse().unwrap(), captures[4].parse().unwrap()),
    })
}

fn solve_p1() -> Result<()> {
    let target = get_input()?;
    dbg!(&target);

    let mut read_line = String::new();

    let mut guess = Point(0, 0);
    let mut prev_guess = guess.clone();
    let mut prev2_guess = guess.clone();
    let mut prev3_guess = guess.clone();
    let mut velocity = guess.clone();
    let mut position = Point(0, 0);
    println!("{:?}", position);
    loop {
        let mut reset = false;
        let prev_position = position.clone();
        position += velocity;
        println!("{:?}\tafter adding {:?}", position, velocity);

        if target.contains(position) {
            println!("HIT!!");
            prev3_guess = prev2_guess;
            prev2_guess = prev_guess;
            prev_guess = guess;
            guess.1 += 1;
            reset = true;
        } else if velocity.1 < 0
            && target.start.1.abs_diff(position.1) > target.start.1.abs_diff(prev_position.1)
        {
            prev3_guess = prev2_guess;
            prev2_guess = prev_guess;
            prev_guess = guess;
            if prev_position.1 < target.start.1 && position.1 > target.end.1
                || prev_position.1 > target.start.1 && position.1 < target.end.1
            {
                println!("Crossed y in one step");
                guess.1 -= 1;
            } else if prev_position.0 < target.start.0 && position.0 > target.end.0 {
                println!("Crossed x in one step");
                guess.1 += 1;
            } else if position.0 < target.start.0 {
                println!("Left of start");
                guess.0 += 1;
            } else if position.0 > target.end.0 {
                println!("Right of end");
                guess.0 -= 1;
            } else {
                println!("In the middle");
                guess.0 += 1;
                guess.1 += 1;
            }
            reset = true;
        }

        if reset {
            println!("\n\nNew Guess {:?}", guess);
            if guess == prev2_guess {
                break;
            }

            std::io::stdin().read_line(&mut read_line).unwrap();
            velocity = guess.clone();
            position = Point(0, 0);
            println!("{:?}", position);
            continue;
        }

        if velocity.0 > 0 {
            velocity.0 -= 1;
        }
        if velocity.0 < 0 {
            velocity.0 += 1;
        }
        velocity.1 -= 1;
    }
    dbg!(prev3_guess);

    Ok(())
}

fn solve_p2() -> Result<()> {
    let input = get_input()?;

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    // solve_p2();

    Ok(())
}
