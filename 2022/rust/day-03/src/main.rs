use std::{collections::HashSet, fs};
fn main() {
    let letters = ('a'..='z')
        .chain('A'..='Z')
        .into_iter()
        .collect::<Vec<char>>();

    let data = "data.txt";
    let file = fs::read_to_string(data).unwrap();
    let rucksacks: Vec<&str> = file.trim().split("\n").collect();

    println!(
        "Sum of priorities: {:?}",
        rucksacks.iter().fold(0, |mut acc, sack| {
            let (first_comp, second_comp) = &sack.split_at(sack.len() / 2);

            acc += first_comp
                .chars()
                .collect::<HashSet<char>>()
                .intersection(&second_comp.chars().collect::<HashSet<char>>())
                .fold(0, |mut acc, el| {
                    acc += letters.iter().position(|&ch| ch == *el).unwrap() + 1;
                    acc
                });

            acc
        })
    );

    println!(
        "Sum of badge priorities: {}",
        rucksacks.chunks_exact(3).fold(0, |mut acc, group| {
            acc += letters
                .iter()
                .position(|&ch| {
                    ch == *group[0]
                        .chars()
                        .collect::<HashSet<char>>()
                        .iter()
                        .find(|&&c| {
                            group[1].chars().collect::<HashSet<char>>().contains(&c)
                                && group[2].chars().collect::<HashSet<char>>().contains(&c)
                        })
                        .unwrap()
                })
                .unwrap()
                + 1;
            acc
        })
    );
}
