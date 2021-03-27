fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let tokens:Vec<char> = line.trim_end().chars().collect();
    println!("{:?}",tokens);
}
