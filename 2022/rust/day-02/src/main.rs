use colored::*;
use std::fs;

fn main() {
    let count = [1, 4, 7, 4, 2].windows(2).fold(0, |mut acc, x| {
        if x.into_iter().sum::<u32>() == 6 {
            acc += 1;
        }

        acc
    });
    let count = [1, 5, 7, 4, 2]
        .windows(2)
        .map(|s| s.into_iter().sum::<u32>())
        .filter(|&n| n == 6)
        .count();
    println!("{}", count);
    let data = "data.txt";
    let game = fs::read_to_string(data).unwrap();
    let game: Vec<&str> = game.trim().split("\n").collect();

    println!(
        "Prediction: {}",
        game.iter()
            .fold(0, |acc, res| {
                acc + match *res {
                    "A X" => 4,
                    "A Y" => 8,
                    "A Z" => 3,

                    "B X" => 1,
                    "B Y" => 5,
                    "B Z" => 9,

                    "C X" => 7,
                    "C Y" => 2,
                    "C Z" => 6,

                    _ => 0,
                }
            })
            .to_string()
            .green()
    );

    println!(
        "Actual: {}",
        game.iter()
            .fold(0, |acc, res| {
                acc + match *res {
                    "A X" => 3,
                    "A Y" => 4,
                    "A Z" => 8,

                    "B X" => 1,
                    "B Y" => 5,
                    "B Z" => 9,

                    "C X" => 2,
                    "C Y" => 6,
                    "C Z" => 7,

                    _ => 0,
                }
            })
            .to_string()
            .green()
    );
}
