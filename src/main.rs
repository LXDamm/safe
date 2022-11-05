use std::env;
use std::process::exit;
use invivofire::Reader;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        exit(-1)
    }
    let command = &args[1];
    match command.as_str() {
        "info" => println!("Running info command"),
        _ => {
            println!("Can't recognize command");
            exit(-1);
        }
    }
    let mut nfc_reader = Reader::new();
    nfc_reader.init();
    exit(0);
}
