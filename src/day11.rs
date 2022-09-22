use color_eyre::eyre::{eyre, Result};
use std::{fmt::Display, fs};

#[derive(Debug)]
struct Octopus {
    energy: u8,
    flashed: bool,
}

impl Octopus {
    fn flash(&mut self) {
        self.energy = 0;
        self.flashed = true;
    }

    fn iteration(&mut self) -> bool {
        if (!self.flashed) {
            self.energy += 1;
            if self.energy > 9 {
                self.flash();
                return true;
            }
        }
        false
    }
}

impl Display for Octopus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.flashed {
            write!(f, "*");
        } else {
            write!(f, " ");
        }
        write!(f, "{}", self.energy)
    }
}

#[derive(Debug)]
struct Grid {
    map: Vec<Vec<Octopus>>,
}

impl Grid {
    fn new(map: Vec<Vec<Octopus>>) -> Self {
        Grid { map }
    }

    fn iteration(&mut self) -> u32 {
        for row in self.map.iter_mut() {
            for octopus in row.iter_mut() {
                octopus.flashed = false;
            }
        }

        let mut total_flashed = 0;
        let mut flashed_indices: Vec<(usize, usize)> = Vec::new();
        for (i, row) in self.map.iter_mut().enumerate() {
            for (j, octopus) in row.iter_mut().enumerate() {
                let flashed = octopus.iteration();
                if flashed {
                    flashed_indices.push((i, j));
                    total_flashed += 1;
                }
            }
        }
        loop {
            let mut new_flashed_indices: Vec<(usize, usize)> = Vec::new();
            for (row, col) in flashed_indices {
                for (row, col) in self.get_adjacent_indices(row, col) {
                    let flashed = self.map[row][col].iteration();
                    if flashed {
                        new_flashed_indices.push((row, col));
                        total_flashed += 1;
                    }
                }
            }

            if new_flashed_indices.len() == 0 {
                break;
            } else {
                flashed_indices = new_flashed_indices;
            }
        }

        total_flashed
    }

    fn get_adjacent_indices(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut adjacent: Vec<(usize, usize)> = Vec::new();

        if row == 0 {
            adjacent.push((row + 1, col));
            if col == 0 {
                adjacent.push((row + 1, col + 1));
                adjacent.push((row, col + 1));
            } else if col == self.map[0].len() - 1 {
                adjacent.push((row, col - 1));
                adjacent.push((row + 1, col - 1));
            } else {
                adjacent.push((row + 1, col + 1));
                adjacent.push((row + 1, col - 1));
                adjacent.push((row, col + 1));
                adjacent.push((row, col - 1));
            }
        } else if row == self.map.len() - 1 {
            adjacent.push((row - 1, col));
            if col == 0 {
                adjacent.push((row - 1, col + 1));
                adjacent.push((row, col + 1));
            } else if col == self.map[0].len() - 1 {
                adjacent.push((row, col - 1));
                adjacent.push((row - 1, col - 1));
            } else {
                adjacent.push((row - 1, col + 1));
                adjacent.push((row - 1, col - 1));
                adjacent.push((row, col + 1));
                adjacent.push((row, col - 1));
            }
        } else {
            adjacent.push((row + 1, col));
            adjacent.push((row - 1, col));
            if col == 0 {
                adjacent.push((row + 1, col + 1));
                adjacent.push((row - 1, col + 1));
                adjacent.push((row, col + 1));
            } else if col == self.map[0].len() - 1 {
                adjacent.push((row, col - 1));
                adjacent.push((row - 1, col - 1));
                adjacent.push((row + 1, col - 1));
            } else {
                adjacent.push((row + 1, col + 1));
                adjacent.push((row + 1, col - 1));
                adjacent.push((row - 1, col + 1));
                adjacent.push((row - 1, col - 1));
                adjacent.push((row, col + 1));
                adjacent.push((row, col - 1));
            }
        }

        adjacent
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.map.iter() {
            for octopus in row {
                write!(f, "{} ", octopus);
            }
            writeln!(f);
        }
        Ok(())
    }
}

fn get_input() -> Result<Grid> {
    let contents = fs::read_to_string("inputs/day11.txt").expect("");
    let lines = contents.split('\n').filter_map(|v| {
        if v.len() == 0 || v.chars().nth(0) == Some('#') {
            None
        } else {
            Some(v)
        }
    });

    Ok(Grid::new(
        lines
            .map(|v| {
                v.chars()
                    .map(|v| Octopus {
                        energy: v as u8 - 0x30,
                        flashed: false,
                    })
                    .collect::<Vec<Octopus>>()
            })
            .collect(),
    ))
}

fn solve_p1() -> Result<()> {
    let mut grid = get_input()?;

    let mut ans = 0;
    for _ in 0..100 {
        ans += grid.iteration();
    }
    dbg!(ans);

    Ok(())
}

pub fn solve_p2() -> Result<()> {
    let mut grid = get_input()?;
    let total_octopodes = grid.map.len() * grid.map[0].len();

    let mut ans = 0;
    for i in 0.. {
        if grid.iteration() == total_octopodes as u32 {
            ans = i + 1;
            break;
        }
    }
    dbg!(ans);

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    solve_p2();

    Ok(())
}
