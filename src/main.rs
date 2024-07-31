use std::{
    env,
    fs::read_to_string,
    io::{self, Read, Stdin, Stdout, Write},
};

struct Brainfuck {
    ip: usize,
    dp: usize,
    data: Vec<u8>,
    stdin: Stdin,
    stdout: Stdout,
}

impl Brainfuck {
    fn new() -> Brainfuck {
        Brainfuck {
            ip: 0,
            dp: 0,
            data: vec![0; 30_000],
            stdin: io::stdin(),
            stdout: io::stdout(),
        }
    }

    fn execute(&mut self, program: &str) {
        let program = program.chars().collect::<Vec<char>>();
        while self.ip < program.len() {
            match program[self.ip] {
                '>' => self.dp += 1,
                '<' => self.dp -= 1,
                '+' => self.data[self.dp] += 1,
                '-' => self.data[self.dp] -= 1,
                '.' => self
                    .stdout
                    .write_all(&self.data[self.dp..self.dp + 1])
                    .expect("output failed"),
                ',' => self
                    .stdin
                    .read_exact(&mut self.data[self.dp..self.dp + 1])
                    .expect("input failed"),
                '[' => {
                    if self.data[self.dp] == 0 {
                        let mut depth = 0;
                        for i in self.ip + 1..program.len() {
                            self.ip += 1;
                            if program[i] == '[' {
                                depth += 1;
                            } else if program[i] == ']' {
                                if depth > 0 {
                                    depth -= 1;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
                ']' => {
                    if self.data[self.dp] != 0 {
                        let mut depth = 0;
                        for i in (0..self.ip).rev() {
                            self.ip -= 1;
                            if program[i] == ']' {
                                depth += 1;
                            } else if program[i] == '[' {
                                if depth > 0 {
                                    depth -= 1;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            self.ip += 1;
        }
    }
}

fn main() {
    let program = env::args().nth(1).expect("expected program file");
    let program = read_to_string(program).expect("failed to read program");
    let mut bf = Brainfuck::new();
    bf.execute(&program);
}

#[cfg(test)]
mod tests {
    use crate::Brainfuck;

    #[test]
    fn simple() {
        let mut bf = Brainfuck::new();
        bf.execute("++++++++");
        assert_eq!(bf.data[0], 8);
    }

    #[test]
    fn skip() {
        let mut bf = Brainfuck::new();
        bf.execute("[+++++++]");
        assert_eq!(bf.data[0], 0);
    }
}
