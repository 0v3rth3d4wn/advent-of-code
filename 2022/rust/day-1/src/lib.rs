use colored::*;
use std::error::Error;
use std::fs;

pub struct Config<'a> {
    pub file_path: &'a String,
}

impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Too few arguments!");
        }

        let file_path = &args[1];
        Ok(Config { file_path })
    }
}

fn get_max_calories(reindeers: &mut Vec<u32>, top: usize) -> u32 {
    if top > 1 && reindeers.len() > 1 {
        reindeers.sort();
        reindeers.drain(0..reindeers.len() - top);
        let top_three: u32 = reindeers.iter().sum();
        return top_three;
    }

    match reindeers.iter().max() {
        Some(&max) => max,
        None => 0,
    }
}

fn reindeers_with_calories(calories: Vec<&str>) -> Vec<u32> {
    let mut cal_sum: u32 = 0;
    let mut reindeers: Vec<u32> = vec![];
    for cal in calories {
        if cal.is_empty() {
            cal_sum = 0;
            continue;
        }
        cal_sum += cal.parse::<u32>().unwrap();
        reindeers.push(cal_sum);
    }

    reindeers
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!(
        "{}{}",
        "Loading calories from ".truecolor(0, 255, 0),
        config.file_path.truecolor(0, 255, 0).bold()
    );

    let file_contents = fs::read_to_string(config.file_path)?;
    let calories: Vec<&str> = file_contents.trim().split("\n").collect();
    let mut reindeers: Vec<u32> = reindeers_with_calories(calories);

    println!(
        "Elf with max calories: {}",
        get_max_calories(&mut reindeers, 1).to_string().green()
    );

    println!(
        "Top 3 elves calories sum: {}",
        get_max_calories(&mut reindeers, 3).to_string().green()
    );

    Ok(())
}
