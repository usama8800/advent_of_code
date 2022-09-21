use color_eyre::eyre::{eyre, Result};
use std::collections::HashMap;
use std::fs;
use std::slice::Windows;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point(i32, i32);
#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
    curr: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        Line {
            start,
            end,
            curr: start,
        }
    }
}

impl Iterator for Line {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.curr;

        if (self.start.0 == self.end.0) {
            let direction = self.end.1 - self.start.1;
            if (direction > 0 && self.curr.1 <= self.end.1) {
                self.curr = Point(self.curr.0, self.curr.1 + 1);
                return Some(ret);
            } else if (direction < 0 && self.curr.1 >= self.end.1) {
                self.curr = Point(self.curr.0, self.curr.1 - 1);
                return Some(ret);
            }
        } else if (self.start.1 == self.end.1) {
            let direction = self.end.0 - self.start.0;
            if (direction > 0 && self.curr.0 <= self.end.0) {
                self.curr = Point(self.curr.0 + 1, self.curr.1);
                return Some(ret);
            } else if (direction < 0 && self.curr.0 >= self.end.0) {
                self.curr = Point(self.curr.0 - 1, self.curr.1);
                return Some(ret);
            }
        } else {
            let direction_x = self.end.0 - self.start.0;
            let direction_y = self.end.1 - self.start.1;

            if (direction_y > 0 && self.curr.1 <= self.end.1
                || direction_y < 0 && self.curr.1 >= self.end.1
                || direction_x > 0 && self.curr.0 <= self.end.0
                || direction_x < 0 && self.curr.0 >= self.end.0)
            {
                let dx = if direction_x > 0 { 1 } else { -1 };
                let dy = if direction_y > 0 { 1 } else { -1 };
                self.curr = Point(self.curr.0 + dx, self.curr.1 + dy);
                return Some(ret);
            }
        }

        None
    }
}

fn get_input() -> Result<Vec<Line>> {
    let contents = fs::read_to_string("inputs/day5.txt").expect("");
    let lines = contents.split('\n').filter_map(|v| {
        if v.len() == 0 || v.chars().nth(0) == Some('#') {
            None
        } else {
            Some(v)
        }
    });

    let ret: Vec<Line> = lines
        .map(|line| {
            line.split(" -> ")
                .map(|v| {
                    v.split(',')
                        .filter_map(|v| v.parse::<i32>().ok())
                        .collect::<Vec<i32>>()
                })
                .filter_map(|v| Some(Point(*v.get(0)?, *v.get(1)?)))
                .collect::<Vec<Point>>()
        })
        .filter_map(|v| Some(Line::new(*v.get(0)?, *v.get(1)?)))
        .collect();

    Ok(ret)
}

fn solve_p1() -> Result<()> {
    let lines = get_input()?;
    let mut grid: HashMap<Point, i32> = HashMap::new();

    for line in lines {
        for point in line {
            let curr = grid.get(&point).map_or(0, |v| *v);
            grid.insert(point, curr + 1);
        }
    }

    let mut ret = 0;
    for (point, num) in grid {
        if num >= 2 {
            ret += 1;
        }
    }
    dbg!(ret);

    Ok(())
}

pub fn solve_p2() -> Result<()> {
    let input = get_input()?;

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    // solve_p2();

    Ok(())
}
