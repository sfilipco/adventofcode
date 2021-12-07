use std::fs;

use anyhow::Result;

const BITS: usize = 12;
const LINES: usize = 1000;

fn counts<'a, L: Iterator<Item = &'a str>>(lines: L, h: char) -> [usize; BITS] {
    let mut cnts = [0usize; BITS];

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == h {
                cnts[i] += 1;
            }
        }
    }
    cnts
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let ones = counts(input.lines(), '1');

    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..BITS {
        if ones[BITS - i - 1] * 2 > LINES {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }
    println!("{}", gamma * epsilon);

    let mut oxygen_candidates = input.lines().collect::<Vec<_>>();
    let mut co2_candidates = oxygen_candidates.clone();
    for i in 0..BITS {
        if oxygen_candidates.len() > 1 {
            let ones = counts(oxygen_candidates.iter().map(|x| *x), '1');
            let length = oxygen_candidates.len();
            oxygen_candidates = oxygen_candidates
                .into_iter()
                .filter(|x| (&x[i..i + 1] == "1") == (ones[i] * 2 >= length))
                .collect();
        }
        if co2_candidates.len() > 1 {
            let ones = counts(co2_candidates.iter().map(|x| *x), '1');
            let length = co2_candidates.len();
            co2_candidates = co2_candidates
                .into_iter()
                .filter(|x| (&x[i..i + 1] == "1") == (ones[i] * 2 < length))
                .collect();
        }
        println!(
            "pos: {}\n{:?}\n{:?}\n",
            i, oxygen_candidates, co2_candidates
        );
    }
    println!(
        "{}",
        i32::from_str_radix(oxygen_candidates[0], 2)? * i32::from_str_radix(co2_candidates[0], 2)?
    );
    Ok(())
}
