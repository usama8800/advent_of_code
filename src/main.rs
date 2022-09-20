#![allow(dead_code, unused)]

use color_eyre::eyre::Result;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() -> Result<()> {
    color_eyre::install()?;

    day4::solve()
}
