use std::{collections::HashSet, fs};

fn parse_buf(buf: &str, mark: usize) -> Option<usize> {
    for i in 0..=buf.len() - mark {
        if &buf[i..i + mark]
            .chars()
            .collect::<HashSet<char>>()
            .into_iter()
            .collect::<Vec<char>>()
            .len()
            == &mark
        {
            return Some(i + mark);
        }
    }
    None
}

fn main() {
    let buf = fs::read_to_string("data.txt")
        .unwrap()
        .trim_end()
        .to_string();
    println!("start-of-packet: {}", parse_buf(&buf, 4).unwrap());
    println!("start-of-message: {}", parse_buf(&buf, 14).unwrap());
}
