use std::{collections::HashSet, fs};

fn main() {
    let data = "data.txt";
    let file = fs::read_to_string(data).unwrap();
    let lines: Vec<&str> = file.trim().split("\n").collect();
    let mut some_overlap: u32 = 0;
    let mut full_overlap_sum: u32 = 0;

    for line in lines {
        let pairs: Vec<&str> = line.split(',').collect();
        let first_pair: Vec<u32> = pairs[0].split('-').flat_map(|x| x.parse::<u32>()).collect();
        let second_pair: Vec<u32> = pairs[1].split('-').flat_map(|x| x.parse::<u32>()).collect();

        if (first_pair[1] >= second_pair[1] && first_pair[0] <= second_pair[0])
            || (second_pair[1] >= first_pair[1] && second_pair[0] <= first_pair[0])
        {
            full_overlap_sum += 1;
        }

        let first_range_set: HashSet<u32> = HashSet::from_iter(first_pair[0]..=first_pair[1]);
        let second_range_set: HashSet<u32> = HashSet::from_iter(second_pair[0]..=second_pair[1]);

        if first_range_set
            .intersection(&second_range_set)
            .collect::<Vec<&u32>>()
            .len()
            > 0
        {
            some_overlap += 1;
        }
    }
    println!("Full overlap: {}", full_overlap_sum);
    println!("Some overlap: {}", some_overlap);
}
