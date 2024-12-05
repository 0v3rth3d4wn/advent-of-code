use std::{collections::HashMap, fs};

fn main() {
    let file = fs::read_to_string("data.txt").unwrap();
    let directions: Vec<char> = file.trim_end().chars().collect();
    let mut santa_houses = vec![(0, 0)];
    let mut robo_santa_houses = vec![(0, 0)];
    let mut houses = vec![(0, 0)];
    let mut houses_count_p1: HashMap<(i32, i32), u32> = HashMap::from([((0, 0), 1)]);
    let mut houses_count_p2: HashMap<(i32, i32), u32> = HashMap::from([((0, 0), 1)]);

    directions.iter().for_each(|d| {
        // push houses for p1
        let last_house = houses.last().unwrap();
        match d {
            '^' => houses.push((last_house.0 + 1, last_house.1)),
            '>' => houses.push((last_house.0, last_house.1 + 1)),
            'v' => houses.push((last_house.0 - 1, last_house.1)),
            '<' => houses.push((last_house.0, last_house.1 - 1)),
            _ => (),
        }

        *houses_count_p1.entry(*houses.last().unwrap()).or_default() += 1;
    });

    for d in directions.chunks(2) {
        let last_santa_house = santa_houses.last().unwrap();
        let last_robo_santahouse = robo_santa_houses.last().unwrap();

        match d[0] {
            '^' => santa_houses.push((last_santa_house.0 + 1, last_santa_house.1)),
            '>' => santa_houses.push((last_santa_house.0, last_santa_house.1 + 1)),
            'v' => santa_houses.push((last_santa_house.0 - 1, last_santa_house.1)),
            '<' => santa_houses.push((last_santa_house.0, last_santa_house.1 - 1)),
            _ => (),
        }

        match d[1] {
            '^' => robo_santa_houses.push((last_robo_santahouse.0 + 1, last_robo_santahouse.1)),
            '>' => robo_santa_houses.push((last_robo_santahouse.0, last_robo_santahouse.1 + 1)),
            'v' => robo_santa_houses.push((last_robo_santahouse.0 - 1, last_robo_santahouse.1)),
            '<' => robo_santa_houses.push((last_robo_santahouse.0, last_robo_santahouse.1 - 1)),
            _ => (),
        }

        *houses_count_p2
            .entry(*santa_houses.last().unwrap())
            .or_default() += 1;
        *houses_count_p2
            .entry(*robo_santa_houses.last().unwrap())
            .or_default() += 1;
    }

    println!("Part 1: {:#?}", houses_count_p1.into_iter().count());
    println!("Part 2: {:#?}", houses_count_p2.into_iter().count());
}
