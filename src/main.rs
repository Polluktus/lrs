use clap::{App, Arg};
use lrs;
use std::process;
fn main() {
    let matches = App::new("lsr")
        .version("0.1")
        .author("Author: polluktus")
        .about("ls alternative wirtten in rust")
        .usage("lrs [FLAGS]")
        .arg(Arg::with_name("all")
            .short("a")
            .long("all")
            .multiple(false)
            .required(false)
            .takes_value(false)
            .help("do not ignore entries starting with ."))
        .arg(Arg::with_name("long")
            .short("l")
            .long("long")
            .multiple(false)
            .required(false)
            .takes_value(false)
            .help("use a long listing format"))
        .arg(Arg::with_name("path")
            .required(false)
            .takes_value(true))
        .get_matches();
    
let config = lrs::Config::new(
    matches.is_present("all"),
   matches.is_present("long"),
   matches.value_of("path"));

   if config.all == false && config.long == false {
        if let Err(e) = lrs::run(&config) {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
        } else if config.all == true && config.long == false {
            if let Err(e) = lrs::run_all(&config) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        } else if config.all == false && config.long == true {
            if let Err(e) = lrs::run_list(&config) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        } else if config.all == true && config.long == true {
            if let Err(e) = lrs::run_all_list(&config) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
}