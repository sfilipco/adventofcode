use std::{fs, usize};

use anyhow::Result;
use text_io::scan;

const SIZE: usize = 1001;

fn main() -> Result<()> {
    let mut chart = Box::new([[0i32; SIZE]; SIZE]);

    for line in fs::read_to_string("input.txt")?.lines() {
        let (x0, y0, x1, y1): (usize, usize, usize, usize);
        scan!(line.bytes() => "{},{} -> {},{}", x0, y0, x1, y1);

        if x0 == x1 {
            for y in y0.min(y1)..=y0.max(y1) {
                chart[x0][y] += 1;
            }
        } else if y0 == y1 {
            for x in x0.min(x1)..=x0.max(x1) {
                chart[x][y0] += 1;
            }
        } else {
            let xs = if x0 < x1 { 1 } else { -1 };
            let ys = if y0 < y1 { 1 } else { -1 };
            for i in 0..=((x0 as i32) - (x1 as i32)).abs() {
                let x = (x0 as i32 + i * xs) as usize;
                let y = (y0 as i32 + i * ys) as usize;
                chart[x][y] += 1
            }
        }
    }

    let mut danger_cnt = 0;
    for i in 0..SIZE {
        for j in 0..SIZE {
            if chart[i][j] > 1 {
                danger_cnt += 1;
            }
        }
    }
    println!("{}", danger_cnt);

    Ok(())
}
