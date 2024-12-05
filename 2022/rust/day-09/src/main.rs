use std::{collections::HashMap, fs};

fn main() {
    let data = "data.txt";
    let file = fs::read_to_string(data).unwrap();

    let moves: Vec<(char, usize)> = file
        .trim_end()
        .lines()
        .map(|m| m.split_at(1))
        .collect::<Vec<(&str, &str)>>()
        .iter_mut()
        .map(|&mut (d, m)| {
            (
                d.parse::<char>().unwrap(),
                m.trim().parse::<usize>().unwrap(),
            )
        })
        .collect();

    let mut head_moves: Vec<(i32, i32)> = vec![(0, 0)];
    let mut tail_moves: Vec<(i32, i32)> = vec![(0, 0)];

    let directions: HashMap<char, (i32, i32)> =
        HashMap::from([('L', (0, -1)), ('R', (0, 1)), ('U', (-1, 0)), ('D', (1, 0))]);

    for (dir, dist) in &moves {
        for _m in 0..*dist {
            let new_pos = (
                head_moves.last().unwrap().0 + directions.get(&dir).unwrap().0,
                head_moves.last().unwrap().1 + directions.get(&dir).unwrap().1,
            );
            head_moves.push(new_pos);

            let curr_tail = tail_moves.last().unwrap();

            if (curr_tail.0 - new_pos.0.abs()).abs() > 1
                && (curr_tail.1 - new_pos.1.abs()).abs() > 1
            {
                tail_moves.push((
                    tail_moves.last().unwrap().0 + directions.get(&dir).unwrap().0,
                    tail_moves.last().unwrap().1 + directions.get(&dir).unwrap().1,
                ))
            } else if curr_tail.0 - new_pos.0.abs() > 1 {
                tail_moves.push((
                    tail_moves.last().unwrap().0 + directions.get(&dir).unwrap().0,
                    tail_moves.last().unwrap().1,
                ))
            } else if curr_tail.1 - new_pos.1.abs() > 1 {
                tail_moves.push((
                    tail_moves.last().unwrap().0,
                    tail_moves.last().unwrap().1 + directions.get(&dir).unwrap().1,
                ))
            }
        }
    }

    println!("{:?}", moves);
    println!("{:?}", head_moves);
    println!();
    println!("{:?}", tail_moves);
}
