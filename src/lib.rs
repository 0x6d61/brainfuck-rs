use std::char;
use std::collections::HashMap;

pub struct BrainFuck {
    pub tape: Vec<i64>,
    pub pc: usize,
    pub ptr: usize,
}

impl BrainFuck {
    pub fn run(&mut self, code: Vec<char>) -> String {
        let mut output = String::new();
        let jumps = jump_search(&code);
        while self.pc < code.len() {
            match code[self.pc] {
                '+' => self.tape[self.ptr] += 1,
                '-' => self.tape[self.ptr] -= 1,
                '>' => self.ptr += 1,
                '<' => {
                    if self.ptr > 0 {
                        self.ptr -= 1
                    } else {
                        panic!("ポインタを0よりマイナスにすることは出来ません。");
                    }
                }
                '.' => {
                    let n = self.tape[self.ptr];
                    let n = char::from_u32(n as u32).unwrap();
                    output = format!("{}{}",output,n);
                },
                ',' => {
                    let mut line = String::new();
                    std::io::stdin().read_line(&mut line).unwrap();
                    let line_chars: Vec<char> = line.trim_end().chars().collect();
                    self.tape[self.ptr] = line_chars[0] as u32 as i64;
                },
                '[' => {
                    if self.tape[self.ptr] == 0 {
                        self.pc = jumps[&self.pc];
                    }
                },
                ']' => {
                    if self.tape[self.ptr] != 0 {
                        self.pc = jumps[&self.pc]
                    }
                }
                t => {
                    panic!("{}文字目 不正な文字です。{}", self.pc + 1, t);
                }
            }
            self.pc += 1;
        }
        output
    }
}

fn jump_search(code: &Vec<char>) -> HashMap<usize,usize> {
    let mut jumps = HashMap::new();
    let mut starts: Vec<usize> = Vec::new();

    for (index, &val) in code.iter().enumerate() {
        if val == '[' {
            starts.push(index);
        } else if val == ']' {
            if starts.len() == 0 {
                panic!("]が多すぎます。");
            }
            let from = starts.pop().unwrap();
            let to = index;
            jumps.insert(from, to);
            jumps.insert(to, from);
        }
    }
    if starts.len() > 0 {
        panic!("[が多すぎます。");
    }
    jumps
}
