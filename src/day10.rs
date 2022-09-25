use color_eyre::eyre::{eyre, Result};
use std::{collections::HashMap, fs, hash::Hash};

fn get_input() -> Result<Vec<String>> {
    let contents = fs::read_to_string("inputs/day10.txt").expect("");
    let lines = contents.split('\n').filter_map(|v| {
        if v.len() == 0 || v.chars().nth(0) == Some('#') {
            None
        } else {
            Some(v.to_owned())
        }
    });

    Ok(lines.collect())
}

fn solve_p1() -> Result<()> {
    let lines = get_input()?;
    let score_map = HashMap::from([('>', 25137u32), ('}', 1197), (']', 57), (')', 3)]);
    let end_map = HashMap::from([('<', '>'), ('{', '}'), ('[', ']'), ('(', ')')]);

    let mut score = 0;
    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        for ch in line.chars() {
            match ch {
                '[' | '(' | '<' | '{' => stack.push(ch),
                ']' | ')' | '>' | '}' => {
                    if let Some(end) = stack.pop() {
                        if *end_map.get(&end).unwrap() != ch {
                            // println!("expected {} but found {}", end, ch);
                            score += score_map.get(&ch).unwrap();
                            break;
                        }
                    }
                }
                _ => panic!("Bad input: {ch}"),
            }
        }
    }
    dbg!(score);

    Ok(())
}

fn solve_p2() -> Result<()> {
    let lines = get_input()?;
    let score_map = HashMap::from([('<', 4u64), ('{', 3), ('[', 2), ('(', 1)]);
    let end_map = HashMap::from([('<', '>'), ('{', '}'), ('[', ']'), ('(', ')')]);

    let mut scores: Vec<u64> = Vec::new();
    'main: for line in lines {
        let mut stack: Vec<char> = Vec::new();
        let mut score = 0;
        for ch in line.chars() {
            match ch {
                '[' | '(' | '<' | '{' => stack.push(ch),
                ']' | ')' | '>' | '}' => {
                    if let Some(end) = stack.pop() {
                        if *end_map.get(&end).unwrap() != ch {
                            // println!("expected {} but found {}", end, ch);
                            continue 'main;
                        }
                    }
                }
                _ => panic!("Bad input: {ch}"),
            }
        }
        while stack.len() > 0 {
            // println!("line incomplete:  {}\n     with stack:  {:?}", line, stack);
            let popped = stack.pop().unwrap();
            score *= 5;
            score += score_map.get(&popped).unwrap();
        }
        if score > 0 {
            scores.push(score);
        }
    }
    scores.sort();
    dbg!(scores[scores.len() / 2]);

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    solve_p2();

    Ok(())
}
