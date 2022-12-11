use std::fs;

fn parse_monkeys(input: Vec<&str>) -> Vec<(u128, Vec<u128>, &str, u128, u128, u128, u128)> {
    let mut monkey_vec: Vec<(u128, Vec<u128>, &str, u128, u128, u128, u128)> = vec![];
    for monkey_cmd in input {
        let monkey: Vec<&str> = monkey_cmd.split("\n").map(|l| l.trim()).collect();

        let monkey_index = monkey[0][..monkey[0].len() - 1]
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<u128>()
            .unwrap();

        let monkey_items = monkey[1].split(':').last().unwrap().trim();

        let operation = monkey[2]
            .split(": ")
            .last()
            .unwrap()
            .split("= ")
            .last()
            .unwrap()
            .split("old ")
            .last()
            .unwrap();

        let test = monkey[3].split("by ").last().unwrap();

        let test_true = monkey[4]
            .split("true: ")
            .last()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap();

        let test_false = monkey[5]
            .split("false: ")
            .last()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap();

        monkey_vec.push((
            monkey_index,
            monkey_items
                .split(",")
                .map(|i| i.trim().parse::<u128>().unwrap())
                .collect(),
            operation,
            test.parse::<u128>().unwrap(),
            test_true.parse::<u128>().unwrap(),
            test_false.parse::<u128>().unwrap(),
            0,
        ))
    }

    monkey_vec
}

fn part_1(mut monkey_vec: Vec<(u128, Vec<u128>, &str, u128, u128, u128, u128)>) {
    for _i in 0..20 {
        for monkey_index in 0..monkey_vec.len() {
            if monkey_vec[monkey_index].1.len() == 0 {
                continue;
            }
            let operation: Vec<&str> = monkey_vec[monkey_index]
                .2
                .split_ascii_whitespace()
                .collect();
            let test = monkey_vec[monkey_index].3;
            let test_true = monkey_vec[monkey_index].4;
            let test_false = monkey_vec[monkey_index].5;

            monkey_vec[monkey_index].6 += monkey_vec[monkey_index].1.len() as u128;
            for _curr_monkey_item in 0..monkey_vec[monkey_index].1.len() {
                let thrw = monkey_vec[monkey_index].1.remove(0);
                let worry_level = match *operation.first().unwrap() {
                    "*" => if *operation.last().unwrap() == "old" {
                        thrw as f64 * thrw as f64 / 3_f64
                    } else {
                        thrw as f64 * operation.last().unwrap().parse::<f64>().unwrap() / 3_f64
                    }
                    .floor(),
                    "+" => if *operation.last().unwrap() == "old" {
                        (thrw as f64 + thrw as f64) / 3_f64
                    } else {
                        (thrw as f64 + operation.last().unwrap().parse::<f64>().unwrap()) / 3_f64
                    }
                    .floor(),
                    _ => 0.0,
                };

                if worry_level as u128 % test == 0 {
                    monkey_vec[test_true as usize].1.push(worry_level as u128);
                } else {
                    monkey_vec[test_false as usize].1.push(worry_level as u128);
                }
            }
        }
    }

    monkey_vec.sort_by_key(|k| k.6);
    monkey_vec.reverse();
    println!("{:#?}", monkey_vec[0].6 * monkey_vec[1].6);
}

fn part_2(mut monkey_vec: Vec<(u128, Vec<u128>, &str, u128, u128, u128, u128)>) {
    let mut destress = 1;

    for monkey_index in 0..monkey_vec.len() {
        destress *= monkey_vec[monkey_index].3;
    }

    for _i in 0..10000 {
        for monkey_index in 0..monkey_vec.len() {
            if monkey_vec[monkey_index].1.len() == 0 {
                continue;
            }
            let operation: Vec<&str> = monkey_vec[monkey_index]
                .2
                .split_ascii_whitespace()
                .collect();
            let test = monkey_vec[monkey_index].3;
            let test_true = monkey_vec[monkey_index].4;
            let test_false = monkey_vec[monkey_index].5;

            monkey_vec[monkey_index].6 += monkey_vec[monkey_index].1.len() as u128;
            for _curr_monkey_item in 0..monkey_vec[monkey_index].1.len() {
                let thrw = monkey_vec[monkey_index].1.remove(0);
                let mut worry_level = match *operation.first().unwrap() {
                    "*" => if *operation.last().unwrap() == "old" {
                        thrw as f64 * thrw as f64
                    } else {
                        thrw as f64 * operation.last().unwrap().parse::<f64>().unwrap()
                    }
                    .floor(),
                    "+" => if *operation.last().unwrap() == "old" {
                        (thrw as f64 + thrw as f64)
                    } else {
                        (thrw as f64 + operation.last().unwrap().parse::<f64>().unwrap())
                    }
                    .floor(),
                    _ => 0.0,
                };

                worry_level = worry_level % destress as f64;
                if worry_level as u128 % test == 0 {
                    monkey_vec[test_true as usize].1.push(worry_level as u128);
                } else {
                    monkey_vec[test_false as usize].1.push(worry_level as u128);
                }
            }
        }
    }

    monkey_vec.sort_by_key(|k| k.6);
    monkey_vec.reverse();
    println!("{:#?}", monkey_vec[0].6 * monkey_vec[1].6);
}

fn main() {
    let data = "data.txt";
    let file = fs::read_to_string(data).unwrap();
    let monkey_list: Vec<&str> = file.trim_end().split("\n\n").collect();

    let monkey_vec: Vec<(u128, Vec<u128>, &str, u128, u128, u128, u128)> =
        parse_monkeys(monkey_list);

    part_1(monkey_vec.clone());
    part_2(monkey_vec);
}
