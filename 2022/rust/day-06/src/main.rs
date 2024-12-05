use std::{collections::HashSet, fs};

fn parse_buf(buf: &str, mark: usize) -> usize {
    buf.chars()
        .collect::<Vec<char>>()
        .windows(mark)
        .enumerate()
        .find(|(_i, b)| b.to_vec().iter().collect::<HashSet<&char>>().len() == mark)
        .unwrap()
        .0
        + mark
}

fn main() {
    let buf = fs::read_to_string("data.txt")
        .unwrap()
        .trim_end()
        .to_string();
    println!("start-of-packet: {:?}", parse_buf(&buf, 4));
    println!("start-of-message: {:?}", parse_buf(&buf, 14));
}
