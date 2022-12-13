use color_eyre::eyre::{eyre, Result};
use num::integer::Roots;
use regex::Regex;
use std::{
    fs,
    ops::{Add, AddAssign},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point(i32, i32);

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

    fn width(&self) -> u32 {
        self.start.0.abs_diff(self.end.0) + 1
    }

    fn height(&self) -> u32 {
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
    let contents = fs::read_to_string("inputs/y21d17.txt").expect("");
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

fn simulate(velocity: &Point, target: &Rect) -> Option<i32> {
    // println!("Simulating {:?}", velocity);
    let mut max_y = i32::MIN;
    let mut velocity = velocity.clone();
    let mut position = Point(0, 0);
    loop {
        let prev_position = position.clone();
        position += velocity;
        // println!("{:?}\tafter adding {:?}", position, velocity);

        if target.contains(position) {
            // println!("HIT!!\n");
            return Some(max_y);
        } else if velocity.1 < 0
            // && target.start.1.abs_diff(position.1) > target.start.1.abs_diff(prev_position.1)
            && position.1 < target.start.1
        {
            // println!("");
            return None;
        }

        if position.1 > max_y {
            max_y = position.1;
        }

        if velocity.0 > 0 {
            velocity.0 -= 1;
        }
        if velocity.0 < 0 {
            velocity.0 += 1;
        }
        velocity.1 -= 1;
    }
}

fn solve_p1() -> Result<()> {
    let target = get_input()?;
    // dbg!(&target);

    let mut read_line = String::new();

    let min_x_landing_inside = ((1.0 + 8.0 * target.start.0 as f32).sqrt() / 2.0) as i32;
    let max_x_landing_inside = ((-1.0 + (1.0 + 8.0 * target.end.0 as f32).sqrt()) / 2.0) as i32;

    let mut max_y = 0;
    let mut consecutive_misses = 0;
    for y in 0.. {
        let mut hit = false;
        for x in min_x_landing_inside..=max_x_landing_inside {
            if let Some(max) = simulate(&Point(x, y), &target) {
                // println!("({}, {}) hit\tmax_y = {}", x, y, max);
                hit = true;
                consecutive_misses = 0;
                if max > max_y {
                    max_y = max;
                }
                break;
            } else {
                // println!("({}, {}) missed", x, y);
            }
        }
        if !hit {
            consecutive_misses += 1;
        }
        if consecutive_misses > 100 {
            break;
        }
    }
    dbg!(max_y);
    Ok(())
}

fn solve_p2() -> Result<()> {
    let target = get_input()?;
    // dbg!(&target);

    let mut read_line = String::new();

    let min_x_landing_inside = ((1.0 + 8.0 * target.start.0 as f32).sqrt() / 2.0) as i32;
    let max_x_inside = target.end.0;
    let min_y_inside = target.start.1;

    let mut ways = 0;
    let mut consecutive_misses = 0;
    for y in min_y_inside.. {
        let mut hit = false;
        for x in min_x_landing_inside..=max_x_inside {
            if let Some(_) = simulate(&Point(x, y), &target) {
                // println!("({}, {}) hit", x, y);
                hit = true;
                consecutive_misses = 0;
                ways += 1;
            } else {
                // println!("({}, {}) missed", x, y);
            }
        }
        if !hit {
            consecutive_misses += 1;
        }
        if consecutive_misses > 100 {
            break;
        }
    }
    dbg!(ways);
    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    solve_p2();

    Ok(())
}
