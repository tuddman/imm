use std::error::Error;
use std::fs;

pub struct Config {
    pub command: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        println!("args of interest: {:?}", &args[1..]);

        if args.len() < 3 {
            return Err("not enough Arguments given");
        }
        Ok(Config {
            command: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("command for {}", config.command);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)?;

    println!("with text: {}", contents);
    Ok(())
}
