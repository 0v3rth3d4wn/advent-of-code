use itertools::Itertools;
use std::{collections::HashSet, fs};

fn main() {
    let file = fs::read_to_string("data.txt").unwrap();
    let lines: Vec<&str> = file.trim_end().split("\n").collect();
    let mut all_paths = Vec::new();
    let mut rocks: HashSet<String> = HashSet::new();

    for path in lines {
        let line = path.split(" -> ");

        for (line_start, line_end) in line.tuple_windows() {
            let line_start = line_start
                .split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let line_end = line_end
                .split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            all_paths.push(((line_start[0], line_start[1]), (line_end[0], line_end[1])));
        }
    }

    // println!("{:?}", all_paths);
    let mut depth = 0;
    for line in all_paths {
        let mut range_x = [line.0 .0, line.1 .0];
        range_x.sort();
        let mut range_y = [line.0 .1, line.1 .1];
        range_y.sort();

        for i in range_x[0]..=range_x[1] {
            for j in range_y[0]..=range_y[1] {
                if j > depth {
                    depth = j;
                }
                // println!("{}:{}", i, j);
                let mut rock_key = String::from(i.to_string());
                rock_key.push_str(":");
                rock_key.push_str(&j.to_string());
                rocks.insert(rock_key);
            }
        }

        println!();
    }

    // println!("depth: {}", depth);
    let sand_ind = 0;

    println!("{:?}", rocks);
}
