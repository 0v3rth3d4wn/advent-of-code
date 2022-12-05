use std::{fs, vec};

fn get_move(mv: &str) -> (usize, usize, usize) {
    let mv_split = mv
        .split(' ')
        .flat_map(|x| x.parse::<usize>())
        .collect::<Vec<usize>>();
    (mv_split[0], mv_split[1] - 1, mv_split[2] - 1)
}

fn parse_crates(crates_raw: &str) -> Vec<Vec<char>> {
    let mut rows: Vec<&str> = crates_raw.split("\n").collect();
    rows.pop();
    rows.reverse();
    let stacks_len = (rows[0].len() + 1) / 4;
    let mut crates: Vec<Vec<char>> = vec![vec![' '; 0]; stacks_len];

    for i in 0..rows.len() {
        let mut crate_index = 0;

        for j in 0..rows[i].len() {
            if j == 1 || j % 4 == 1 {
                let ch = rows[i].chars().nth(j).unwrap();
                if ch != ' ' {
                    crates[crate_index].push(rows[i].chars().nth(j).unwrap());
                }
                crate_index += 1;
            }
        }
    }

    crates
}

fn main() {
    let data = "data.txt";
    let file = fs::read_to_string(data).unwrap();
    let (crates, moves) = file.split_at(file.find("\n\n").unwrap());
    let moves: Vec<&str> = moves.trim().split("\n").collect();

    let mut crates_9000 = parse_crates(crates);
    for mv in &moves {
        let (mv_times, mv_from, mv_to) = get_move(mv);

        for _ in 0..mv_times {
            let s_mv = crates_9000[mv_from].pop().unwrap();
            crates_9000[mv_to].push(s_mv);
        }
    }

    println!(
        "Crate Mover 9000: {:?}",
        crates_9000
            .iter()
            .fold(String::from(""), |mut acc: String, c| {
                acc.push_str(&c.last().unwrap().to_string());
                acc
            })
    );

    let mut crates_9001 = parse_crates(crates);
    for mv in &moves {
        let (mv_times, mv_from, mv_to) = get_move(mv);

        if mv_times > 1 {
            let crate_len = crates_9001[mv_from].len();
            let mut bulk_move = crates_9001[mv_from]
                .drain(crate_len - mv_times..crate_len)
                .collect::<Vec<char>>();
            crates_9001[mv_to].append(&mut bulk_move);
        } else {
            let s_mv = crates_9001[mv_from].pop().unwrap();
            crates_9001[mv_to].push(s_mv);
        }
    }

    println!(
        "Crate Mover 9001: {:?}",
        crates_9001
            .iter()
            .fold(String::from(""), |mut acc: String, c| {
                acc.push_str(&c.last().unwrap().to_string());
                acc
            })
    );
}
