/* Use */
use std::error::Error;
use std::fs;

/* Struct Config */
pub struct Config {
    pub query: String,
    pub file_path: String
}

/* Impl Config */
impl Config {
    /* Parse arguments */
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        /* Check arguments */
        if args.len() < 3 {
            return Err("Not enought arguments");
        }

        /* Arguments */
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }    
}

/* Run */
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    /* Read file */
    let contents = fs::read_to_string(config.file_path)?;

    /* Out content */
    println!("\n{contents}\n");

    Ok(())
}