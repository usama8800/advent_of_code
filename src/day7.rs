use color_eyre::eyre::{eyre, Result};
use std::fs;
use std::slice::Windows;

fn get_input() -> Result<Vec<u32>> {
    let contents = fs::read_to_string("inputs/day7.txt").expect("");
    let mut lines = contents.split('\n').filter_map(|v| {
        if v.len() == 0 || v.chars().nth(0) == Some('#') {
            None
        } else {
            Some(v)
        }
    });
    let line = lines.next().ok_or_else(|| eyre!(""))?.split(',');
    Ok(line.filter_map(|v| v.parse().ok()).collect())
}

fn optimal<F>(input: &Vec<u32>, fuel_calc: F) -> Result<()>
where
    F: Fn(u32, u32) -> u32,
{
    let mut steps: Vec<Vec<u32>> = vec![input.iter().map(|v| fuel_calc(*v, 0)).collect()];
    let mut prev_sum: u32 = steps.get(0).ok_or_else(|| eyre!(""))?.iter().sum();

    loop {
        let step_num = steps.len();
        steps.push(steps.get(step_num - 1).ok_or_else(|| eyre!(""))?.clone());
        let step = steps.get_mut(step_num).ok_or_else(|| eyre!(""))?;
        for i in 0..input.len() {
            let input_num = *input.get(i).ok_or_else(|| eyre!(""))?;
            let moves = fuel_calc(input_num, step_num as u32);
            let step_ref = step.get_mut(i).ok_or_else(|| eyre!(""))?;
            *step_ref = moves;
        }
        let curr_sum: u32 = step.iter().sum();
        if curr_sum > prev_sum {
            break;
        }
        prev_sum = curr_sum;
    }
    dbg!(prev_sum);
    Ok(())
}

fn solve_p1() -> Result<()> {
    let input = get_input()?;
    optimal(&input, |a, b| a.abs_diff(b))
}

pub fn solve_p2() -> Result<()> {
    let input = get_input()?;
    optimal(&input, |x: u32, y: u32| {
        let diff = x.abs_diff(y);
        (1..=diff).sum::<u32>()
    })
}

pub fn solve() -> Result<()> {
    solve_p1();
    solve_p2();

    Ok(())
}
