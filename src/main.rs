use brainfuck_rs::BrainFuck;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let tokens = if args.len() > 1 {
        let mut f = File::open(&args[1]).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        let tokens:Vec<char> = contents.chars().filter(|&s|s != '\n').collect();
        tokens

    } else {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let tokens: Vec<char> = line.trim_end().chars().collect();
        tokens
    };
    let mut bf = BrainFuck {
        tape: vec![0; 10000],
        pc: 0,
        ptr: 0,
    };
    let output = bf.run(tokens);
    println!("{}", output);
}
