use regex::Regex;
use std::collections::{HashMap, HashSet};

enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}
struct Instruction {
    action: Action,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}
trait InstructionsParser {
    fn parse(lines: &Vec<&str>) -> Vec<Instruction> {
        let re = Regex::new(r"^([a-z|\s]+)\s(\d+),(\d+)\s[a-z]+\s(\d+),(\d+)").unwrap();
        lines
            .iter()
            .map(|l| {
                let caps = re
                    .captures(l)
                    .map(|cap| {
                        cap.iter()
                            .skip(1)
                            .map(|c| c.unwrap().as_str())
                            .collect::<Vec<&str>>()
                    })
                    .unwrap();
                let action = match caps[0] {
                    "turn off" => Action::TurnOff,
                    "turn on" => Action::TurnOn,
                    "toggle" => Action::Toggle,
                    _ => panic!("Unrecognized action"),
                };
                let x1 = caps[1].parse::<usize>().unwrap();
                let y1 = caps[2].parse::<usize>().unwrap();
                let x2 = caps[3].parse::<usize>().unwrap();
                let y2 = caps[4].parse::<usize>().unwrap();

                Instruction {
                    action,
                    x1,
                    y1,
                    x2,
                    y2,
                }
            })
            .collect::<Vec<Instruction>>()
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Point(usize, usize);

#[derive(Eq, Hash, PartialEq)]
struct Bulb {
    coord: Point,
}

struct Bulb9000 {
    coord: Point,
    brightness: usize,
}

struct Board {
    bulbs: HashSet<Bulb>,
}

impl Board {
    fn new() -> Self {
        Self {
            bulbs: HashSet::new(),
        }
    }

    fn create(instructions: &Vec<&str>) -> Self {
        let mut board = Board::new();
        Board::parse(instructions).iter().for_each(|i| {
            let Instruction {
                action,
                x1,
                y1,
                x2,
                y2,
            } = i;

            for x in *x1..=*x2 {
                for y in *y1..=*y2 {
                    match action {
                        Action::TurnOn => board.turn_on_bulb(Bulb { coord: Point(x, y) }),
                        Action::TurnOff => board.turn_off_bulb(Bulb { coord: Point(x, y) }),
                        Action::Toggle => board.toggle(Bulb { coord: Point(x, y) }),
                    };
                }
            }
        });
        Self { bulbs: board.bulbs }
    }

    fn turn_on_bulb(&mut self, bulb: Bulb) {
        self.bulbs.insert(bulb);
    }

    fn turn_off_bulb(&mut self, bulb: Bulb) {
        self.bulbs.remove(&bulb);
    }

    fn toggle(&mut self, bulb: Bulb) {
        if !self.bulbs.remove(&bulb) {
            self.bulbs.insert(bulb);
        }
    }

    fn len(&self) -> usize {
        self.bulbs.len()
    }
}

impl InstructionsParser for Board {}

fn do_lights(
    action: &Action,
    lights: &mut HashSet<(usize, usize)>,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            match action {
                Action::TurnOn => lights.insert((x, y)),
                Action::TurnOff => lights.remove(&(x, y)),
                Action::Toggle => {
                    if !lights.remove(&(x, y)) {
                        lights.insert((x, y));
                    }
                    true
                }
            };
        }
    }
}

fn do_brights(
    action: &Action,
    brights: &mut HashMap<(usize, usize), usize>,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            let bulb = brights.entry((x, y)).or_default();
            match action {
                Action::TurnOn => *bulb += 1,
                Action::TurnOff => {
                    if *bulb > 0 {
                        *bulb -= 1
                    }
                }
                Action::Toggle => *bulb += 2,
            };
        }
    }
}

fn main() {
    let file = std::fs::read_to_string("data.txt").unwrap();
    let lines = file.trim_end().split("\n").collect::<Vec<&str>>();
    let re = Regex::new(r"^([a-z|\s]+)\s(\d+),(\d+)\s[a-z]+\s(\d+),(\d+)").unwrap();
    let mut lights: HashSet<(usize, usize)> = HashSet::new();
    let mut brights: HashMap<(usize, usize), usize> = HashMap::new();

    // let board = Board::create(&lines);
    // println!("Board: {}", board.len());

    for instr in lines {
        let caps = re
            .captures(instr)
            .map(|cap| {
                cap.iter()
                    .skip(1)
                    .map(|c| c.unwrap().as_str())
                    .collect::<Vec<&str>>()
            })
            .unwrap();

        let action = match caps[0] {
            "turn off" => Action::TurnOff,
            "turn on" => Action::TurnOn,
            "toggle" => Action::Toggle,
            _ => panic!("Unrecognized action"),
        };
        let x1 = caps[1].parse::<usize>().unwrap();
        let y1 = caps[2].parse::<usize>().unwrap();
        let x2 = caps[3].parse::<usize>().unwrap();
        let y2 = caps[4].parse::<usize>().unwrap();

        do_lights(&action, &mut lights, x1, y1, x2, y2);
        do_brights(&action, &mut brights, x1, y1, x2, y2);
    }

    println!("Part 1: {}", lights.len());
    println!(
        "Part 2: {}",
        brights.iter().fold(0, |mut acc, b| {
            acc += b.1;
            acc
        })
    );
}
