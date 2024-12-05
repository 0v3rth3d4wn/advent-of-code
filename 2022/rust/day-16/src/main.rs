use std::{
    collections::{HashMap, HashSet},
    fs,
};
fn main() {
    let file = fs::read_to_string("data.txt").unwrap();
    let lines: Vec<&str> = file.trim_end().split("\n").collect();
    // key, (flow_rate, leads_to)
    let mut valves: HashMap<&str, (u32, Vec<&str>)> = HashMap::new();
    for line in lines {
        let line_split = line.split_once(';').unwrap();
        let key_flow = line_split.0.split_whitespace().collect::<Vec<&str>>();
        let valve_key = key_flow[1];
        let valve_flow_rate = key_flow
            .last()
            .unwrap()
            .split_once('=')
            .unwrap()
            .1
            .trim_end_matches(';');
        let valve_leads_to = line_split
            .1
            .trim()
            .splitn(5, ' ')
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .split(", ")
            .collect::<Vec<&str>>();

        valves.insert(
            valve_key,
            (valve_flow_rate.parse::<u32>().unwrap(), valve_leads_to),
        );
    }
}
