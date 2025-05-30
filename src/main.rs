mod ast;
mod lexer;
mod parse;
mod play;
mod run;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("wrong!");
        println!("dolce <main.dc>");
        return;
    }

    let src = match fs::read_to_string(&args[1]) {
        Ok(f) => f,
        Err(_) => {
            println!("can't read file");
            println!("dolce <main.dc>");
            return;
        }
    };

    let sheet = lexer::tokenize(src);
    let music = parse::parse(sheet);
    play::play(music);
}
