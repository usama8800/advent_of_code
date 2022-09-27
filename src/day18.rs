use color_eyre::eyre::{eyre, Result};
use std::{fmt::Display, fs, ops::Add};

#[derive(Debug)]
enum NumberPart {
    Literal(u8),
    Number(Number),
}

#[derive(Debug)]
struct Number {
    left: Box<NumberPart>,
    right: Box<NumberPart>,
}

impl Number {
    fn new(str: &str) -> Self {
        let str = str.strip_prefix('[').unwrap();
        let str = str.strip_suffix(']').unwrap();
        let mut brackets = 0;
        for (i, char) in str.char_indices() {
            if char == '[' {
                brackets += 1;
            }
            if char == ']' {
                brackets -= 1;
            }
            if char == ',' && brackets == 0 {
                let split: (&str, &str) = (
                    &str.chars().take(i).collect::<String>(),
                    &str.chars().skip(i + 1).collect::<String>(),
                );
                let left = if let Ok(x) = split.0.parse::<u8>() {
                    Box::new(NumberPart::Literal(x))
                } else {
                    Box::new(NumberPart::Number(Number::new(split.0)))
                };
                let right = if let Ok(x) = split.1.parse::<u8>() {
                    Box::new(NumberPart::Literal(x))
                } else {
                    Box::new(NumberPart::Number(Number::new(split.1)))
                };
                return Number { left, right };
            }
        }
        unreachable!("{}", str);
    }

    fn find_deepest(&self) -> String {
        let left = if let NumberPart::Number(num) = self.left.as_ref() {
            let mut ret = String::from("0");
            ret.push_str(&num.find_deepest());
            ret
        } else {
            "".to_owned()
        };

        let right = if let NumberPart::Number(num) = self.right.as_ref() {
            let mut ret = String::from("1");
            ret.push_str(&num.find_deepest());
            ret
        } else {
            "".to_owned()
        };

        if left.len() > right.len() {
            left
        } else {
            right
        }
    }

    fn reduce(&mut self) {
        loop {
            let path = self.find_deepest();
            if path.len() < 4 {
                break;
            }
            let mut r: Option<&mut Number> = Some(self);
            let mut literal: Option<&mut NumberPart> = None;
            for i in 0..path.len() - 1 {
                if &path[i..=i] == "0" {
                    let rr = r.unwrap();
                    if let NumberPart::Number(num) = rr.left.as_mut() {
                        r = Some(num);
                    } else {
                        panic!();
                    }
                    if let num @ NumberPart::Literal(_) = rr.right.as_mut() {
                        literal = Some(num);
                    }
                }
                if &path[i..=i] == "1" {
                    let rr = r.unwrap();
                    if let NumberPart::Number(num) = rr.right.as_mut() {
                        r = Some(num);
                    } else {
                        panic!();
                    }
                    if let num @ NumberPart::Literal(_) = rr.left.as_mut() {
                        literal = Some(num);
                    }
                }
            }
            let last_char = path.chars().last().unwrap();
            let second_last = r.unwrap();
            if last_char == '0' {
                if let NumberPart::Number(num) = second_last.left.as_ref() {
                    if let NumberPart::Literal(inner_left) = num.left.as_ref() {
                        if let NumberPart::Literal(inner_right) = num.right.as_ref() {
                            if let NumberPart::Literal(outer_right) = second_last.right.as_ref() {
                                second_last.right =
                                    Box::new(NumberPart::Literal(inner_right + outer_right));
                                second_last.left = Box::new(NumberPart::Literal(0));
                            } else {
                                panic!();
                            }
                        } else {
                            panic!();
                        }
                    } else {
                        panic!();
                    }
                } else {
                    panic!();
                }
            }
            if last_char == '1' {
                if let NumberPart::Number(last) = second_last.right.as_ref() {
                    if let NumberPart::Literal(inner_left) = last.left.as_ref() {
                        if let c @ NumberPart::Literal(inner_right) = last.right.as_ref() {
                            dbg!(c);
                            if let Some(NumberPart::Literal(outer_right)) = literal {
                                *outer_right += *inner_right;
                            } else {
                                panic!();
                            }
                            if let NumberPart::Literal(outer_left) = second_last.left.as_ref() {
                                second_last.left =
                                    Box::new(NumberPart::Literal(inner_left + outer_left));
                                second_last.right = Box::new(NumberPart::Literal(0));
                            } else {
                                panic!();
                            }
                        } else {
                            panic!();
                        }
                    } else {
                        panic!();
                    }
                } else {
                    panic!();
                }
            }
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut num = Self {
            left: Box::from(NumberPart::Number(self)),
            right: Box::from(NumberPart::Number(rhs)),
        };
        num.reduce();
        num
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[");

        match self.left.as_ref() {
            NumberPart::Literal(lit) => write!(f, "{}", lit),
            NumberPart::Number(num) => write!(f, "{}", num),
        };
        write!(f, ",");
        match self.right.as_ref() {
            NumberPart::Literal(lit) => write!(f, "{}", lit),
            NumberPart::Number(num) => write!(f, "{}", num),
        };

        write!(f, "]");
        Ok(())
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
    let mut input = get_input()?;
    // dbg!(input);
    let mut x = &mut input[0];
    x.reduce();
    println!("{}", x);

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
