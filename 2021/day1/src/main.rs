use anyhow::{anyhow, Result};
use std::fs;

fn main() -> Result<()> {
    let items: Vec<i32> = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| i32::from_str_radix(line.trim(), 10))
        .collect::<Result<Vec<i32>, _>>()?;

    let mut sums = items.windows(3).map(|window| window.iter().sum());
    let mut prev: i32 = sums.next().ok_or_else(|| anyhow!("Empty iterator"))?;
    let mut counter = 0;
    for sum in sums {
        if sum > prev {
            counter = counter + 1;
        }
        prev = sum;
    }
    println!("{}", counter);

    Ok(())
}
