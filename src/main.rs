use std::env;
use std::process;

use rain::input::{Config, Data, Rawinput};
use rain::solve::categorise;
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

    let problem = Problem::new(data.duration, &data.profile);

    println!("{:?}", problem);
    println!("{:?}", categorise(problem));
}
