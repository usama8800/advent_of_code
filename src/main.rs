#![allow(dead_code, unused)]

use color_eyre::eyre::Result;

mod day18;
use day18 as day;

fn main() -> Result<()> {
    color_eyre::install()?;

    day::solve()
}
