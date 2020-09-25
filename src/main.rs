use std::env;
use std::process;

use rain::input::{Config, Data, Rawinput};
use rain::solutions;
use rain::Problem;

fn main() {
    // get config from comand line arguments
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args, "example.toml");

    // read raw input
    let rawinput = Rawinput::new(config).unwrap_or_else(|err| {
        eprintln!("Input file not found: {}", err);
        process::exit(1);
    });

    // parse toml to struct Data
    let data = Data::new(rawinput).unwrap_or_else(|err| {
        eprintln!("Input .toml cannot be parsed: {}", err);
        process::exit(1);
    });

    // pre-process data and get struct describing problem
    let problem = Problem::new(data.duration, &data.profile);

    // this provides the adequate function to solve a given problem
    let solver = solutions::select_fn(&problem);

    // calculate results and print them
    println!("Resulting water and ground levels:");
    println!("{:?}", solver(problem).levels);
}
