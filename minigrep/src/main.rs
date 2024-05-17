use std::env::args;
use std::process::exit;
use minigrep::Config;

fn main() {
    let args: Vec<String> = args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        exit(1);
    });
    
    if let Err(e) = minigrep::run(config){
        eprintln!("Allication error: {e}");
        exit(1);
    };
}


