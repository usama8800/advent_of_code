#![allow(dead_code, unused)]

use color_eyre::eyre::Result;

mod day14;
use day14 as day;

fn main() -> Result<()> {
    color_eyre::install()?;

    day::solve()
}
