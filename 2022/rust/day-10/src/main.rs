use grid::{grid, Grid};
use std::fs;

fn maybe_inc_str(cycle: i32, strength: &mut Vec<i32>, reg: &i32) -> () {
    let should_increase_strength = match cycle {
        20 => true,
        c if c > 20 && (c - 20) % 40 == 0 => true,
        _ => false,
    };

    if should_increase_strength == true {
        strength.push(cycle * reg);
    }
}

fn main() {
    let data = "data.txt";
    let file = fs::read_to_string(data).unwrap();
    let instructions = file.trim_end().lines();
    let mut cycles: i32 = 0;
    let mut reg: i32 = 1;
    let mut str: Vec<i32> = vec![];
    // let mut sprite: (isize, isize, isize) = (1, 2, 3);
    // let mut grid = Grid::from_vec(vec!['.'; 240], 40);
    // let mut curr_row = 0;
    for line in instructions {
        let inst: Vec<&str> = line.split_terminator(" ").collect();
        match inst[0] {
            "noop" => {
                cycles += 1;
                maybe_inc_str(cycles, &mut str, &reg);
            }
            "addx" => {
                for _i in 0..=1 {
                    cycles += 1;
                    maybe_inc_str(cycles, &mut str, &reg);
                }
                reg += inst[1].parse::<i32>().unwrap();
            }
            _ => (),
        }
    }
    println!("str: {:?}", str.iter().sum::<i32>());

    // for line in instructions {
    //     let inst: Vec<&str> = line.split_terminator(" ").collect();
    //     match inst[0] {
    //         "noop" => {
    //             cycles += 1;
    //             if sprite.0 == cycles as isize
    //                 || sprite.1 == cycles as isize
    //                 || sprite.2 == cycles as isize
    //             {
    //                 grid[curr_row][cycles - 1] = '#';
    //             }
    //             if cycles == 240 {
    //                 break;
    //             }
    //         }
    //         "addx" => {
    //             for _i in 0..=1 {
    //                 cycles += 1;

    //                 if sprite.0 == cycles as isize
    //                     || sprite.1 == cycles as isize
    //                     || sprite.2 == cycles as isize
    //                 {
    //                     grid[curr_row][cycles - 1 as usize] = '#';
    //                 }
    //                 if cycles == 240 {
    //                     break;
    //                 }
    //             }

    //             sprite.0 += inst[1].parse::<isize>().unwrap();
    //             sprite.1 += inst[1].parse::<isize>().unwrap();
    //             sprite.2 += inst[1].parse::<isize>().unwrap();
    //         }
    //         _ => (),
    //     }

    //     if cycles % 40 == 0 {
    //         curr_row += 1;
    //     }
    // }

    // println!("{:#?}", grid);
}
