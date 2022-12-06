use std::fs;

fn parse_buf(buf: &str, mark: usize) -> Option<usize> {
    for i in 0..=buf.len() - mark {
        let batch = &buf[i..i + mark];
        if batch.chars().find(|c| batch.matches(*c).count() > 1) == None {
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
