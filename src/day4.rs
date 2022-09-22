use color_eyre::eyre::{eyre, Result};
use std::fmt::Display;
use std::fs;

#[derive(Debug)]
struct BingoNum {
    num: u32,
    marked: bool,
}

#[derive(Debug)]
struct Bingo {
    rows: Vec<Vec<BingoNum>>,
}

impl Display for Bingo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for num in row {
                write!(f, "{:>2}", num.num);
                if num.marked {
                    write!(f, "*  ");
                } else {
                    write!(f, "   ");
                }
            }
            writeln!(f, "");
        }
        Ok(())
    }
}

impl Bingo {
    fn new(rows: Vec<Vec<u32>>) -> Bingo {
        Bingo {
            rows: rows
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|num| BingoNum { marked: false, num })
                        .collect()
                })
                .collect(),
        }
    }

    fn mark(&mut self, to_mark: u32) {
        for row in self.rows.iter_mut() {
            for num in row {
                if num.num == to_mark {
                    num.marked = true
                }
            }
        }
    }

    fn is_bingo(&self) -> Option<bool> {
        'outer: for row in &self.rows {
            for num in row {
                if !num.marked {
                    continue 'outer;
                }
            }
            return Some(true);
        }
        'outer: for col in 0..self.rows.get(0)?.len() {
            for row in 0..self.rows.len() {
                if !self.rows.get(row)?.get(col)?.marked {
                    continue 'outer;
                }
            }
            return Some(true);
        }
        Some(false)
    }

    fn score(&self, latest: u32) -> u32 {
        let unmarked_sum: u32 = self
            .rows
            .iter()
            .map(|v| v.iter().filter(|v| !v.marked).map(|v| v.num).sum::<u32>())
            .sum();
        unmarked_sum * latest
    }
}

fn get_input() -> Result<(Vec<u32>, Vec<Bingo>)> {
    let contents = fs::read_to_string("inputs/day4.txt").expect("");
    let mut splits = contents
        .split('\n')
        .filter_map(|v| if v.len() == 0 { None } else { Some(v) });

    let draws_str = splits.next().ok_or_else(|| eyre!(""))?;
    let draws_splits = draws_str.split(',');
    let draws: Vec<u32> = draws_splits.filter_map(|v| v.parse().ok()).collect();

    let board_strs: Vec<&str> = splits.collect();
    let boards: Vec<Bingo> = board_strs
        .chunks_exact(5)
        .map(|v| {
            Bingo::new(
                v.into_iter()
                    .map(|line| {
                        line.split_ascii_whitespace()
                            .filter_map(|v| v.parse::<u32>().ok())
                            .collect::<Vec<u32>>()
                    })
                    .collect::<Vec<Vec<u32>>>(),
            )
        })
        .collect();

    Ok((draws, boards))
}

fn solve_p1() -> Result<()> {
    let (draws, mut boards) = get_input()?;

    for draw in draws {
        for board in &mut boards {
            board.mark(draw);
            if board.is_bingo().ok_or_else(|| eyre!(""))? {
                println!("{}", board.score(draw));
                println!("{}", board);
                return Ok(());
            }
        }
    }

    Ok(())
}

pub fn solve_p2() -> Result<()> {
    let (draws, mut boards) = get_input()?;

    for draw in draws {
        let mut removing: Vec<usize> = vec![];
        let last = boards.len() == 1;
        for i in 0..boards.len() {
            let mut board = boards.get_mut(i).ok_or_else(|| eyre!(""))?;
            board.mark(draw);
            if board.is_bingo().ok_or_else(|| eyre!(""))? {
                if last {
                    println!("{}", board.score(draw));
                    println!("{}", board);
                    return Ok(());
                }
                removing.push(i);
            }
        }
        for (i, remove) in removing.iter().enumerate() {
            boards.remove(remove - i);
        }
    }

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    solve_p2();

    Ok(())
}
