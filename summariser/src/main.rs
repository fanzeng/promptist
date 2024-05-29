use std::env;
use std::process;

use summariser::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // let args: Vec<String> = env::args().collect();
    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(e) = summariser::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
