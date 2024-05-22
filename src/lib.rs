use std::error::Error;
use std::fs;
use std::io::stdout;
use std::io::Write;

pub struct Config {
    _file: String,
    search: String,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        match args.len() {
            4.. => return Err("Too much arguments"),

            ..=2 => return Err("Too few arguments"),

            3 => stdout()
                .write_all(b"Waiting for text...\n")
                .expect("Argument Error"),
        };
        let file = &args[1].clone();
        let search = &args[2].clone();
        Ok(Config {
            _file: file.to_string(),
            search: search.to_string(),
        })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config._file)?;
    println!("{content}");
    Ok(())
}
