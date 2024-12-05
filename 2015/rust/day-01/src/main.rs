use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

use std::fs;

fn main() {
    let file = fs::read_to_string("data.txt").unwrap();
    let file = file.trim_end();
    println!(
        "Part 1: {}",
        file.chars().fold(0, |mut acc, i| {
            if i == '(' {
                acc += 1;
            } else {
                acc -= 1;
            }

            acc
        })
    );

    println!(
        "Part 2: {}",
        file.chars()
            .enumerate()
            .fold_while(0, |mut acc: isize, (i, c)| {
                if c == '(' {
                    acc += 1;
                } else {
                    acc -= 1;
                }

                if acc == -1 {
                    Done((i + 1) as isize)
                } else {
                    Continue(acc)
                }
            })
            .into_inner()
    );
}
