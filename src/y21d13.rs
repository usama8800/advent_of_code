use color_eyre::eyre::{eyre, Result};
use std::{collections::HashSet, fmt::Display, fs};

#[derive(Debug)]
enum Command {
    FoldX(u32),
    FoldY(u32),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Grid {
    points: Vec<Point>,
    width: u32,
    height: u32,
}

impl Grid {
    fn new(points: Vec<Point>) -> Self {
        let max_x = points.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
        let max_y = points.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
        Grid {
            points,
            width: max_x + 1,
            height: max_y + 1,
        }
    }

    fn fold(&mut self, command: &Command) {
        // println!("{:?} {:>4} x {:>4}", command, self.height, self.width);
        match command {
            Command::FoldX(x) => {
                for point in self.points.iter_mut() {
                    if point.x > *x {
                        point.x = self.width - point.x - self.width % 2;
                    }
                }
                self.width /= 2;
            }
            Command::FoldY(y) => {
                for point in self.points.iter_mut() {
                    if point.y > *y {
                        point.y = self.height - point.y - self.height % 2;
                    }
                }
                self.height /= 2;
            }
        }
        let set: HashSet<Point> = HashSet::from_iter(self.points.clone().into_iter());
        self.points = set.into_iter().collect();
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                let point = self.points.iter().find(|v| v.x == j && v.y == i);
                if let Some(point) = point {
                    write!(f, "#");
                } else {
                    write!(f, ".");
                }
            }
            writeln!(f);
        }

        Ok(())
    }
}

fn get_input() -> Result<(Grid, Vec<Command>)> {
    let contents = fs::read_to_string("inputs/y21d13.txt").expect("");
    let lines = contents.split('\n').filter_map(|v| {
        if v.len() == 0 || v.chars().nth(0) == Some('#') {
            None
        } else {
            Some(v)
        }
    });

    let mut points: Vec<Point> = Vec::new();
    let mut commands: Vec<Command> = Vec::new();
    for line in lines {
        if line.starts_with("fold along") {
            let line = line.strip_prefix("fold along ").unwrap();
            let splits: Vec<&str> = line.split('=').collect();
            if splits[0] == "y" {
                commands.push(Command::FoldY((splits[1].parse().unwrap())));
            } else {
                commands.push(Command::FoldX((splits[1].parse().unwrap())));
            }
        } else {
            let splits: Vec<u32> = line.split(',').map(|v| v.parse().unwrap()).collect();
            points.push(Point {
                x: splits[0],
                y: splits[1],
            });
        }
    }

    let mut grid = Grid::new(points);
    Ok((grid, commands))
}

fn solve_p1() -> Result<()> {
    let (mut grid, commands) = get_input()?;
    // println!("{}", grid);
    grid.fold(&commands[0]);
    // println!("{}", grid);
    // grid.fold(&commands[1]);
    // println!("{}", grid);
    println!("{}", grid.points.len());

    Ok(())
}

fn solve_p2() -> Result<()> {
    let (mut grid, commands) = get_input()?;

    for command in commands {
        grid.fold(&command);
    }
    println!("{}", grid);

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    solve_p2();

    Ok(())
}
