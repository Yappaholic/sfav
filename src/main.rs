use sfav::run;
use sfav::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error getting data: {err}");
        process::exit(1)
    });
    if let Err(e) = run(config) {
        println!("Error occured: {e}");
        process::exit(1);
    }
}
