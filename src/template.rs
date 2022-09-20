use color_eyre::eyre::Result;
use std::fs;
use std::slice::Windows;

fn get_input() -> Result<Vec<u32>> {
    let contents = fs::read_to_string("inputs/dayX.txt").expect("");
    let splits = contents.split('\n');

    Ok(splits.filter_map(|v| v.parse().ok()).collect())
}

fn solve_p1() -> Result<()> {
    let nums = get_input()?;

    Ok(())
}

pub fn solve_p2() -> Result<()> {
    let nums = get_input()?;

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    solve_p2();

    Ok(())
}
