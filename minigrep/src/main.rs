use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

struct Config {
    query: String,
    filename: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let conf: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem Parsing Arguments: {err}");
        process::exit(1);
    });

    let mut f: File = File::open(conf.filename).expect("file not found");

    let mut contents: String = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{contents}")
}

impl Config {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query: String = args[1].clone();
        let filename: String = args[2].clone();
        Ok(Self { query, filename })
    }
}
