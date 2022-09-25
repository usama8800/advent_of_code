use color_eyre::eyre::{eyre, Result};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::Chars;

#[derive(Debug, Default)]
struct Digit {
    a: Option<char>,
    b: Option<char>,
    c: Option<char>,
    d: Option<char>,
    e: Option<char>,
    f: Option<char>,
    g: Option<char>,
}

impl Digit {
    fn figure_connections(&mut self, connections: &String) {
        let mut try_digits: Vec<&str> = connections.split(' ').collect();
        try_digits.sort_by(|a, b| a.len().cmp(&b.len()));

        let sets: Vec<HashSet<char>> = try_digits
            .iter()
            .map(|v| HashSet::from_iter(v.chars()))
            .collect();
        self.a = Some(*sets[0].symmetric_difference(&sets[1]).next().unwrap());
        let mut bd: Vec<&char> = sets[2].difference(&sets[0]).collect();
        let eight = sets.iter().find(|v| v.len() == 7).unwrap();
        let zero = sets
            .iter()
            .find(|v| {
                v.len() == 6
                    && (v.contains(&bd[0]) ^ v.contains(&bd[1]))
                    && sets[0].difference(v).count() == 0
            })
            .unwrap();
        self.d = Some(*eight.symmetric_difference(zero).next().unwrap());
        bd.retain(|v| *v != &self.d.unwrap());
        self.b = Some(*bd[0]);
        let nine = sets
            .iter()
            .find(|v| {
                v.difference(&sets[0]).count() == 4
                    && sets[0].difference(v).count() == 0
                    && v.contains(&self.d.unwrap())
            })
            .unwrap();

        self.e = Some(*nine.symmetric_difference(eight).next().unwrap());
        let six = sets
            .iter()
            .find(|v| v.difference(&sets[0]).count() == 5 && v.len() == 6)
            .unwrap();
        self.c = Some(*sets[0].difference(six).next().unwrap());
        self.f = Some(*sets[0].iter().find(|v| **v != self.c.unwrap()).unwrap());
        self.g = Some(
            *eight
                .iter()
                .find(|v| {
                    ![
                        self.a.unwrap(),
                        self.b.unwrap(),
                        self.c.unwrap(),
                        self.d.unwrap(),
                        self.e.unwrap(),
                        self.f.unwrap(),
                    ]
                    .contains(*v)
                })
                .unwrap(),
        );
    }

    fn what_is(&self, connections: &String) -> Vec<u8> {
        let try_digits: Vec<String> = connections
            .split(' ')
            .map(|v| {
                let mut chars = v
                    .chars()
                    .map(|v| {
                        let digit = Some(v);
                        if digit == self.a {
                            return 'a';
                        } else if digit == self.b {
                            return 'b';
                        } else if digit == self.c {
                            return 'c';
                        } else if digit == self.d {
                            return 'd';
                        } else if digit == self.e {
                            return 'e';
                        } else if digit == self.f {
                            return 'f';
                        } else if digit == self.g {
                            return 'g';
                        }
                        panic!("Bad input");
                    })
                    .collect::<Vec<char>>();
                chars.sort();
                chars.iter().collect::<String>()
            })
            .collect();

        let mut ret = Vec::new();
        for digit in try_digits {
            match digit.as_str() {
                "abcefg" => ret.push(0),
                "cf" => ret.push(1),
                "acdeg" => ret.push(2),
                "acdfg" => ret.push(3),
                "bcdf" => ret.push(4),
                "abdfg" => ret.push(5),
                "abdefg" => ret.push(6),
                "acf" => ret.push(7),
                "abcdefg" => ret.push(8),
                "abcdfg" => ret.push(9),
                _ => panic!("Bad digit: {}", digit),
            }
        }
        ret
    }
}

fn get_input() -> Result<Vec<Vec<String>>> {
    let contents = fs::read_to_string("inputs/day8.txt").expect("");
    let lines = contents.split('\n').filter_map(|v| {
        if v.len() == 0 || v.chars().nth(0) == Some('#') {
            None
        } else {
            Some(v)
        }
    });

    let ret = lines
        .map(|line| {
            line.split(" | ")
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    Ok(ret)
}

fn solve_p1() -> Result<()> {
    let input = get_input()?;

    let mut ans = 0;
    for line in input {
        let final_digits = &line[1];
        let mut digit: Digit = Default::default();
        digit.figure_connections(&line[0]);
        let digits = digit.what_is(&line[1]);

        for digit in digits {
            if [1u8, 4, 7, 8].contains(&digit) {
                ans += 1;
            }
        }
    }
    dbg!(ans);

    Ok(())
}

fn solve_p2() -> Result<()> {
    let input = get_input()?;

    let mut ans = 0;
    for line in input {
        let final_digits = &line[1];
        let mut digit: Digit = Default::default();
        digit.figure_connections(&line[0]);
        let digits = digit.what_is(&line[1]);
        let mut num: u32 = 0;
        for i in 0..digits.len() {
            num += digits[i] as u32 * 10u32.pow(digits.len() as u32 - i as u32 - 1);
        }
        ans += num;
    }
    dbg!(ans);

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    solve_p2();

    Ok(())
}
