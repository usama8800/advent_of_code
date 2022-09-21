#![allow(dead_code, unused)]

use color_eyre::eyre::Result;

mod day7;
use day7 as day;

fn main() -> Result<()> {
    color_eyre::install()?;

    day::solve()
}
