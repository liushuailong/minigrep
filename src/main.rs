use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    // let query = &args[1];
    // let file_name = &args[2];
    let (query, file_name) = parse_config(&args);
    println!("Searching for {}", query);
    println!("In file {}", file_name);

    let contents = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_name = &args[2];
    (query, file_name)
}
