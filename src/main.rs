use brainfuck_rs::BrainFuck;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let tokens:Vec<char> = line.trim_end().chars().collect();
    let mut bf = BrainFuck {
        tape:vec![0;10000],
        pc:0,
        ptr:0,
    };
    let output = bf.run(tokens);
    println!("{}",output);
}
