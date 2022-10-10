/* Use */
use std::process;
use std::env;

/*  Connection data from other file
    use project_name::data
*/
use n14_input_output::Config;

/* Console:
    cargo run
    cargo run -- searchstring example-filename.txt
    cargo run -- needle haystack
    cargo run -- test poem.txt
*/

/* Entry point */
fn main() {
    /* Collection arguments from the console */
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    /* Parse arguments */
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    /* Out arguments */
    println!("Search {}", config.query);
    println!("In file {}", config.file_path);

    /* Run */
    if let Err(e) = n14_input_output::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}