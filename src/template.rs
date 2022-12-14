use color_eyre::eyre::{eyre, Result};
use std::fs;

fn get_input() -> Result<Vec<u32>> {
    let contents = fs::read_to_string("inputs/y22dX.txt").expect("");
    let lines = contents.split('\n').filter_map(|v| {
        if v.len() == 0 || v.chars().nth(0) == Some('#') {
            None
        } else {
            Some(v)
        }
    });

    Ok(Vec::new())
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
    // solve_p2();

    Ok(())
}
