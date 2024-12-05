use std::fs;

fn main() {
    let file = fs::read_to_string("data.txt").unwrap();
    let presents: Vec<&str> = file.trim_end().split("\n").collect();

    println!(
        "Part 1: {}",
        presents.iter().fold(0, |mut acc, p| {
            let dimensions: Vec<u32> = p.split("x").map(|s| s.parse::<u32>().unwrap()).collect();

            let sides = vec![
                dimensions[0] * dimensions[1],
                dimensions[1] * dimensions[2],
                dimensions[2] * dimensions[0],
            ];

            acc += (sides[0] * 2 + sides[1] * 2 + sides[2] * 2) + sides.iter().min().unwrap();
            acc
        })
    );

    println!(
        "Part 2: {}",
        presents.iter().fold(0, |mut acc, p| {
            let mut dimensions: Vec<u32> =
                p.split("x").map(|s| s.parse::<u32>().unwrap()).collect();
            dimensions.sort();

            acc += dimensions[0] * 2
                + dimensions[1] * 2
                + dimensions[0] * dimensions[1] * dimensions[2];
            acc
        })
    );
}
