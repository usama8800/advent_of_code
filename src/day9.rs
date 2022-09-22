use color_eyre::eyre::{eyre, Result};
use std::fs;

#[derive(Debug)]
struct Grid {
    map: Vec<Vec<u8>>,
    curr_i: usize,
}

impl Grid {
    fn new(map: Vec<Vec<u8>>) -> Self {
        Grid { curr_i: 0, map }
    }

    fn iter(&self) -> GridIter<'_> {
        GridIter {
            grid: self,
            curr_i: 0,
        }
    }

    fn get_adjacent(&self, row: usize, col: usize) -> Vec<u8> {
        let mut adjacent: Vec<u8> = Vec::new();

        if row == 0 {
            adjacent.push(self.map[row + 1][col]);
        } else if row == self.map.len() - 1 {
            adjacent.push(self.map[row - 1][col]);
        } else {
            adjacent.push(self.map[row + 1][col]);
            adjacent.push(self.map[row - 1][col]);
        }

        if col == 0 {
            adjacent.push(self.map[row][col + 1]);
        } else if col == self.map[0].len() - 1 {
            adjacent.push(self.map[row][col - 1]);
        } else {
            adjacent.push(self.map[row][col + 1]);
            adjacent.push(self.map[row][col - 1]);
        }

        adjacent
    }

    fn get_basin_size(&self, row: usize, col: usize) -> u32 {
        let mut done: Vec<(usize, usize)> = Vec::new();

        fn helper(
            map: &Vec<Vec<u8>>,
            row: usize,
            col: usize,
            done: &mut Vec<(usize, usize)>,
        ) -> u32 {
            if row >= map.len()
                || col >= map[0].len()
                || map[row][col] == 9
                || done.contains(&(row, col))
            {
                return 0;
            }

            done.push((row, col));

            1 + if row > 0 {
                helper(map, row - 1, col, done)
            } else {
                0
            } + if col > 0 {
                helper(map, row, col - 1, done)
            } else {
                0
            } + helper(map, row, col + 1, done)
                + helper(map, row + 1, col, done)
        }

        helper(&self.map, row, col, &mut done)
    }
}

struct GridIter<'a> {
    curr_i: usize,
    grid: &'a Grid,
}

impl<'a> Iterator for GridIter<'a> {
    type Item = (usize, usize, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.curr_i / self.grid.map[0].len();
        let col = self.curr_i % self.grid.map[0].len();
        self.curr_i += 1;

        if row == self.grid.map.len() {
            None
        } else {
            Some((row, col, self.grid.map[row][col]))
        }
    }
}

fn get_input() -> Result<Grid> {
    let contents = fs::read_to_string("inputs/day9.txt").expect("");
    let lines = contents.split('\n').filter_map(|v| {
        if v.len() == 0 || v.chars().nth(0) == Some('#') {
            None
        } else {
            Some(v)
        }
    });

    Ok(Grid::new(
        lines
            .map(|v| v.chars().map(|v| v as u8 - 0x30).collect::<Vec<u8>>())
            .collect(),
    ))
}

fn solve_p1() -> Result<()> {
    let grid = get_input()?;

    let mut ans: u32 = 0;
    'main: for (row, col, num) in grid.iter() {
        let adjacents = grid.get_adjacent(row, col);
        for adjacent in adjacents {
            if adjacent <= num {
                continue 'main;
            }
        }
        // println!("{} {} {}", row, col, num);
        ans += 1 + num as u32;
    }

    dbg!(ans);

    Ok(())
}

pub fn solve_p2() -> Result<()> {
    let grid = get_input()?;

    let mut sizes: Vec<u32> = Vec::new();
    'main: for (row, col, num) in grid.iter() {
        let adjacents = grid.get_adjacent(row, col);
        for adjacent in adjacents {
            if adjacent <= num {
                continue 'main;
            }
        }
        let basin_size = grid.get_basin_size(row, col);
        // println!("{} {} {}", row, col, basin_size);
        sizes.push(basin_size);
    }
    sizes.sort();
    sizes.reverse();

    let ans = sizes[0] * sizes[1] * sizes[2];

    dbg!(ans);

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    solve_p2();

    Ok(())
}
