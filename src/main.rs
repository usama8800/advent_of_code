#![allow(dead_code, unused)]

use color_eyre::eyre::Result;

mod day1;

fn main() -> Result<()> {
    color_eyre::install()?;

    day1::solve()
}
