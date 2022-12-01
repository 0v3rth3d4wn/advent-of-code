use colored::*;
use day_1::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!(
            "{}{}",
            "Problem parsing arguments:\n".red(),
            err.red().bold()
        );
        process::exit(1);
    });

    if let Err(e) = day_1::run(config) {
        println!(
            "{}{}",
            "Application error: ".red(),
            e.to_string().red().bold()
        );
        process::exit(1);
    }
}
