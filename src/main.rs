use std::{
    env,
    fs::read_to_string,
    io::{self, Read, Write},
};

struct Brainfuck<'a, R: Read, W: Write> {
    ip: usize,
    dp: usize,
    data: Vec<u8>,
    read: &'a mut R,
    write: &'a mut W,
}

impl<'a, R: Read, W: Write> Brainfuck<'a, R, W> {
    fn new(read: &'a mut R, write: &'a mut W) -> Brainfuck<'a, R, W> {
        Brainfuck {
            ip: 0,
            dp: 0,
            data: vec![0; 30_000],
            read,
            write,
        }
    }

    fn execute(&mut self, program: &str) {
        let program = program.chars().collect::<Vec<char>>();
        while self.ip < program.len() {
            match program[self.ip] {
                '>' => self.dp += 1,
                '<' => self.dp -= 1,
                '+' => self.data[self.dp] = self.data[self.dp].wrapping_add(1),
                '-' => self.data[self.dp] = self.data[self.dp].wrapping_sub(1),
                '.' => self
                    .write
                    .write_all(&self.data[self.dp..self.dp + 1])
                    .expect("output failed"),
                ',' => self
                    .read
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
    let mut input = io::stdin();
    let mut output = io::stdout();
    let mut bf = Brainfuck::new(&mut input, &mut output);
    bf.execute(&program);
}

#[cfg(test)]
mod tests {
    use std::io;

    use crate::Brainfuck;

    #[test]
    fn hello_world() {
        let mut input = io::stdin();
        let mut output = Vec::new();
        let mut bf = Brainfuck::new(&mut input, &mut output);
        bf.execute("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
        assert_eq!(output, b"Hello World!\n");
    }
}
