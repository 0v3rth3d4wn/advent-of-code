use std::{
    collections::{HashMap, HashSet},
    fs, vec,
};

fn main() {
    let data = "data.txt";
    let file = fs::read_to_string(data).unwrap();
    let mut current_path = Vec::new();
    let mut sizes: HashMap<(String, String), usize> = HashMap::new();
    let mut dirs: HashMap<String, usize> = HashMap::new();
    let lines: Vec<&str> = file.trim_end().split("\n").collect();

    for line in lines {
        let cmd: Vec<&str> = line.split(" ").collect();

        if cmd[0] == "$" && cmd[1] == "cd" {
            current_path = match cmd[2] {
                "/" => vec![""],
                ".." => current_path.drain(..current_path.len() - 1).collect(),
                _ => vec![current_path, vec![cmd[2]]].concat(),
            };
        }

        if let Ok(n) = cmd[0].parse::<usize>() {
            let mut dir_name = current_path.clone().join("/");
            dir_name.push_str("/");
            dirs.insert(dir_name.clone(), 0);
            // dir_name.push_str(cmd[1]);
            sizes.insert((dir_name, cmd[1].to_string()), n);
        }
    }

    let mut sums: Vec<usize> = vec![0; dirs.len()];

    let mut i = 0;
    for size in &sizes {
        for dir in &dirs {
            if size.0 .0.starts_with(dir.0) {
                sums[i] += size.1;
            };
            i += 1;
        }

        i = 0;
    }

    println!("{:#?}", dirs);
    println!("{:#?}", sizes);
    println!("{:#?}", sums);

    let mut mega_sum = 0;
    for sum in sums {
        if sum <= 100000 {
            mega_sum += sum;
        }
    }

    println!("{}", mega_sum);
}
