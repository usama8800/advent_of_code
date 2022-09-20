use color_eyre::eyre::Result;
use std::fs;
use std::slice::Windows;

fn get_input() -> Result<Vec<u32>> {
    let contents = fs::read_to_string("inputs/day1.txt")?;
    let splits = contents.split_ascii_whitespace();

    Ok(splits.filter_map(|v| v.parse().ok()).collect())
}

fn solve_p1() -> Result<()> {
    let ret = get_input()?.into_iter().fold((0, std::u32::MAX), |acc, v| {
        if v > acc.1 {
            return (acc.0 + 1, v);
        } else {
            return (acc.0, v);
        }
    });
    dbg!(ret.0);

    Ok(())
}

pub fn solve_p2() -> Result<()> {
    let nums = get_input()?;
    let mut ret = 0;
    for x in nums.windows(4) {
        match x {
            [a, b, c, d] => ret += if d > a { 1 } else { 0 },
            _ => unreachable!(),
        }
    }
    dbg!(ret);

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    solve_p2();

    Ok(())
}
