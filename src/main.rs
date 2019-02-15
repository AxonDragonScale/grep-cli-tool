use std::env;
use std::process;

use grep_cli_tool;
use grep_cli_tool::Config;

fn main() {
    // env::args() returns an iterator, collect() collects the elements in it and returns a vector
    // we need to specify so that collect knows what type to collect into
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args)

    let cli_args = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing the arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {} in {}", cli_args.query, cli_args.filename);

    if let Err(e) = grep_cli_tool::run(cli_args) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
