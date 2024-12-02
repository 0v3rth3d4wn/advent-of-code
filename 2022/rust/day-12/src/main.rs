use pathfinding::prelude::bfs;
use std::fs;

fn find_index(lines: &Vec<&str>, c: char) -> (usize, usize) {
    for row in 0..lines.len() {
        if let Some(col) = lines[row].find(c) {
            return (row, col);
        }
    }

    (0, 0)
}

fn main() {
    let file = fs::read_to_string("data.txt").unwrap();
    let lines: Vec<&str> = file.trim_end().split("\n").collect();
    let start_index = find_index(&lines, 'S');
    let end_index = find_index(&lines, 'E');

    // let grid = lines.iter().map(|l| l.as_bytes()).collect::<Vec<&[u8]>>();

    let result = bfs(
        &start_index,
        |&(x, y)| {
            vec![
                (x + 1, y + 0),
                (x - 1, y + 0),
                (x + 0, y + 1),
                (x + 0, y - 1),
            ]
        },
        |&p| p == end_index,
    );
    println!("{:?}", result);
    // println!("{:?}", end_index);

    // [83, 97, 98, 113, 112, 111, 110, 109]
    // [97, 98, 99, 114, 121, 120, 120, 108]
    // [97, 99, 99, 115, 122, 69, 120, 107]
    // [97, 99, 99, 116, 117, 118, 119, 106]
    // [97, 98, 100, 101, 102, 103, 104, 105]
}
