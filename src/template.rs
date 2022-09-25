use color_eyre::eyre::{eyre, Result};
use std::fs;

fn get_input() -> Result<Vec<u32>> {
    let contents = fs::read_to_string("inputs/dayX.txt").expect("");
    let lines = contents.split('\n').filter_map(|v| {
        if v.len() == 0 || v.chars().nth(0) == Some('#') {
            None
        } else {
            Some(v)
        }
    });

    Ok(lines.filter_map(|v| v.parse().ok()).collect())
}

fn solve_p1() -> Result<()> {
    let input = get_input()?;
    dbg!(input);

    Ok(())
}

fn solve_p2() -> Result<()> {
    let input = get_input()?;

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    solve_p2();

    Ok(())
}
