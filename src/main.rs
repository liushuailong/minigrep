use std::env;
use std::process;

use ripgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments:{}", err);
        process::exit(1);
    });

    // let contents = fs::read_to_string(config.filename)
    //     .expect("Something went wrong reading the file");
    // println!("With text:\n{}", contents);
    if let Err(e) = ripgrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}
