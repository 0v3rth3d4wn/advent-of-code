use std::fs;
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
            let (first_c, second_c) = sack.split_at(sack.len() / 2);
            let first_c_items: Vec<char> = first_c.chars().collect();
            let second_c_items: Vec<char> = second_c.chars().collect();

            for item in second_c_items {
                if first_c_items.contains(&item) {
                    acc += letters.iter().position(|&ch| ch == item).unwrap() + 1;
                    break;
                }
            }
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
                        .collect::<Vec<char>>()
                        .iter()
                        .find(|&&c| {
                            group[1].chars().collect::<Vec<char>>().contains(&c)
                                && group[2].chars().collect::<Vec<char>>().contains(&c)
                        })
                        .unwrap()
                })
                .unwrap()
                + 1;
            acc
        })
    );
}
