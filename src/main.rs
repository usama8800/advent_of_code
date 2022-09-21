#![allow(dead_code, unused)]

use color_eyre::eyre::Result;

mod day6;
use day6 as day;

fn main() -> Result<()> {
    color_eyre::install()?;

    day::solve()
}
