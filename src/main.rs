#![allow(dead_code, unused)]

use color_eyre::eyre::Result;

mod y21d1;
use y21d1 as day;

fn main() -> Result<()> {
    color_eyre::install()?;

    day::solve()
}
