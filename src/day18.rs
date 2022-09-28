use color_eyre::eyre::{eyre, Result};
use std::{fmt::Display, fs};

#[derive(Debug)]
struct Number {
    nums: Vec<(u32, u8)>,
}

impl Number {
    fn new(string: &str) -> Self {
        let mut vec = Vec::new();
        let mut depth = 0;
        for ch in string.chars() {
            if ch == '[' {
                depth += 1;
            } else if ch == ']' {
                depth -= 1;
            } else if ch == ',' {
                continue;
            } else {
                vec.push((ch as u32 - 0x30, depth - 1));
            }
        }
        let mut ret = Self { nums: vec };

        ret.reduce();
        ret
    }

    fn reduce(&mut self) {
        loop {
            // println!("{}", self);
            let mut exploded = false;
            for i in 0..self.nums.len() - 1 {
                if self.nums[i].1 >= 4 {
                    // println!("{:?} {:?}", self.nums[i], self.nums[i + 1]);
                    if i > 0 {
                        self.nums[i - 1].0 += self.nums[i].0;
                    }
                    if i + 2 < self.nums.len() {
                        self.nums[i + 2].0 += self.nums[i + 1].0;
                    }
                    self.nums[i].0 = 0;
                    self.nums[i].1 -= 1;
                    self.nums.remove(i + 1);
                    exploded = true;
                    break;
                }
            }
            if exploded {
                continue;
            }

            let mut insert = None;
            for i in 0..self.nums.len() {
                if self.nums[i].0 > 9 {
                    insert = Some((i, self.nums[i].0 / 2, self.nums[i].1 + 1));
                    self.nums[i].0 = (self.nums[i].0 + 1) / 2;
                    self.nums[i].1 += 1;
                    break;
                }
            }
            if let Some((i, val, depth)) = insert {
                self.nums.insert(i, (val, depth));
                continue;
            }

            break;
        }
    }

    fn magnitude(&self) -> u32 {
        let mut nums = self.nums.clone();
        while nums.len() > 1 {
            let mut max = 0;
            let mut max_i = 0;
            for (i, (_, depth)) in nums.iter().enumerate() {
                if *depth >= max {
                    max = *depth;
                    max_i = i;
                }
            }
            nums[max_i - 1].1 = nums[max_i - 1].1.saturating_sub(1);
            nums[max_i - 1].0 = nums[max_i - 1].0 * 3 + nums[max_i].0 * 2;
            nums.remove(max_i);
        }
        nums[0].0
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.nums)
    }
}

fn get_input() -> Result<Vec<Number>> {
    let contents = fs::read_to_string("inputs/day18.txt").expect("");
    let lines = contents.split('\n').filter_map(|v| {
        if v.len() == 0 || v.chars().nth(0) == Some('#') {
            None
        } else {
            Some(v)
        }
    });

    Ok(lines.map(|line| Number::new(line)).collect())
}

fn solve_p1() -> Result<()> {
    let nums = get_input()?;
    let mut ans: Option<Number> = None;
    for y in nums {
        ans = Some(if let Some(x) = ans {
            let nums = [x.nums, y.nums]
                .iter()
                .flatten()
                .map(|(num, depth)| (*num, *depth + 1))
                .collect();
            let mut num = Number { nums };
            num.reduce();
            num
        } else {
            y
        });
    }
    println!("{}", ans.unwrap().magnitude());

    Ok(())
}

fn solve_p2() -> Result<()> {
    let nums = get_input()?;

    let mut max = 0;
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }

            let sum = {
                let nums = [&nums[i].nums, &nums[j].nums]
                    .iter()
                    .map(|v| *v)
                    .flatten()
                    .map(|(num, depth)| (*num, *depth + 1))
                    .collect();
                let mut num = Number { nums };
                num.reduce();
                num
            };
            let magnitude = sum.magnitude();
            if magnitude > max {
                max = magnitude;
            }
        }
    }
    dbg!(max);

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    solve_p2();

    Ok(())
}
