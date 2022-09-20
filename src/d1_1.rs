use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("inputs/d1.txt")?;
    let splits = contents.split_ascii_whitespace();

    let ret = splits.fold((0, std::u32::MAX), |acc, v| {
        if let Ok(num) = v.parse() {
            if num > acc.1 {
                return (acc.0 + 1, num);
            } else {
                return (acc.0, num);
            }
        } else {
            return acc;
        }
    });
    // for split in splits {
    //     println!("{:?}", split);
    // }
    println!("{:?}", ret.0);

    Ok(())
}
