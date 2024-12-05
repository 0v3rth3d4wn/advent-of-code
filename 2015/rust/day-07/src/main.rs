use std::collections::HashMap;

fn main() {
    let file = std::fs::read_to_string("data.txt").unwrap();
    let lines = file.trim_end().split("\n").collect::<Vec<&str>>();
    let mut wires: HashMap<&str, u16> = HashMap::new();

    for line in lines.iter().cycle() {
        let wire = line.split_once(" -> ");
        if let Ok(i) = wire.unwrap().0.parse::<u16>() {
            // Direct assignment
            wires.insert(wire.unwrap().1, i);
        }

        let left = wire
            .unwrap()
            .0
            .split_ascii_whitespace()
            .collect::<Vec<&str>>();

        if left.len() == 2 {
            // Bitwise NOT
            // NOT kx
            let left_not = wires.get(left[1]);
            if let Ok(i) = left[1].parse::<u16>() {
                if wires.get(left[1]) != None {
                    wires.insert(wire.unwrap().1, !i);    
                }
            }
        }

        if left.len() == 3 {
            // let left_wire = wires.get(left[0]);
            // let right_wire = wires.get(left[2]);

            match left[1] {
                "AND" => if left[1]
                "OR" => 
                "LSHIFT" => if left_wire != None {
                    wires.insert(wire.unwrap().1, left_wire.unwrap() << right_wire.unwrap());
                },
                "RSHIFT" => if left_wire != None {
                    wires.insert(wire.unwrap().1, left_wire.unwrap() >> right_wire.unwrap());
                }
                _ => panic!("Not matched")
            }
            //

            // if left_wire != None && right_wire != None {
            //     wires.insert(wire.unwrap().1, left_wire.unwrap())
            // }
        }

        println!(
            "{:?}",
            wire.unwrap()
                .0
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .len()
        );
    }

    // println!("{:?}", wire.unwrap().0.parse::<u16>());
    println!("{:?}", wires);
}
