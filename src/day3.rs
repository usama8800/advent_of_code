use color_eyre::eyre::{eyre, Result};
use std::fs;

fn get_input() -> Result<Vec<String>> {
    let contents = fs::read_to_string("inputs/day3.txt").expect("");
    let splits = contents.split('\n');

    Ok(splits
        .filter_map(|v| {
            if v.len() > 0 {
                Some(v.to_owned())
            } else {
                None
            }
        })
        .collect())
}

fn solve_p1() -> Result<()> {
    let nums = get_input()?;
    let len = nums.get(0).ok_or_else(|| eyre!(""))?.len();
    let mut ones = vec![0; len];

    for bits in &nums {
        for (i, ch) in bits.char_indices() {
            if ch == '1' {
                ones[i] += 1;
            }
        }
    }

    let gamma_vec: Vec<u32> = ones
        .iter()
        .map(|&v| if v > nums.len() / 2 { 1 } else { 0 })
        .rev()
        .collect();
    let epsilon_vec: Vec<u32> = gamma_vec.iter().map(|v| 1 - v).collect();

    let gamma: u32 = gamma_vec
        .iter()
        .enumerate()
        .map(|(i, &v)| v * 2u32.pow(i as u32))
        .sum();

    let epsilon: u32 = epsilon_vec
        .iter()
        .enumerate()
        .map(|(i, v)| v * 2u32.pow(i as u32))
        .sum();

    dbg!(gamma, epsilon, gamma * epsilon);

    Ok(())
}

fn solve_p2() -> Result<()> {
    let nums = get_input()?;
    let len = nums.get(0).ok_or_else(|| eyre!(""))?.len();

    let mut oxygen_vec = nums.clone();
    let mut i = 0;
    let oxygen_string = loop {
        let mut ones = 0;
        let len = oxygen_vec.len();
        for bits in &oxygen_vec {
            let ch = bits.chars().nth(i).ok_or_else(|| eyre!(""))?;
            if ch == '1' {
                ones += 1;
            }
        }
        oxygen_vec = oxygen_vec
            .into_iter()
            .filter(|v| v.chars().nth(i).unwrap() == if ones >= (len + 1) / 2 { '1' } else { '0' })
            .collect();
        if (oxygen_vec.len() == 1) {
            break oxygen_vec.get(0).unwrap();
        }
        i += 1;
    };

    let mut co2_vec = nums.clone();
    let mut i = 0;
    let co2_string = loop {
        let mut ones = 0;
        let len = co2_vec.len();
        for bits in &co2_vec {
            let ch = bits.chars().nth(i).ok_or_else(|| eyre!(""))?;
            if ch == '1' {
                ones += 1;
            }
        }
        co2_vec = co2_vec
            .into_iter()
            .filter(|v| v.chars().nth(i).unwrap() == if ones > (len - 1) / 2 { '0' } else { '1' })
            .collect();
        if (co2_vec.len() == 1) {
            break co2_vec.get(0).unwrap();
        }
        i += 1;
    };

    let oxygen = u32::from_str_radix(oxygen_string, 2)?;
    let co2 = u32::from_str_radix(co2_string, 2)?;
    dbg!(oxygen, co2, oxygen * co2);

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    solve_p2();

    Ok(())
}
