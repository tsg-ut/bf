use std::collections::{BTreeMap, VecDeque};

pub struct Program {
    instructions: Vec<Instr>,
    mapping: BTreeMap<usize, usize>,
}

enum Token {
    Inc, // +
    Dec, // -
    Bwd, // <
    Fwd, // >
    Opn, // [
    Cls, // ]
    Get, // ,
    Put, // .
}

enum Instr {
    Plus(u8),    // + -
    Step(isize), // < >
    Opn,         // [
    Cls,         // ]
    Get,         // ,
    Put,         // .
}

fn tokenize(code: &String) -> Vec<Token> {
    let mut tokens = Vec::new();
    for c in code.chars() {
        match c {
            '+' => tokens.push(Token::Inc),
            '-' => tokens.push(Token::Dec),
            '<' => tokens.push(Token::Bwd),
            '>' => tokens.push(Token::Fwd),
            '[' => tokens.push(Token::Opn),
            ']' => tokens.push(Token::Cls),
            ',' => tokens.push(Token::Get),
            '.' => tokens.push(Token::Put),
            _ => (),
        }
    }
    tokens
}

pub fn compile(code: &String) -> Result<Program, &str> {
    let tokens = tokenize(&code);
    let mut instructions = Vec::new();
    let mut mapping = BTreeMap::new();
    let mut brackets = VecDeque::new();
    for t in tokens.iter() {
        match t {
            Token::Inc => {
                if let Some(Instr::Plus(plus)) = instructions.last_mut() {
                    if plus.wrapping_add(1) == 0 {
                        instructions.pop();
                    } else {
                        *plus = plus.wrapping_add(1);
                    }
                } else {
                    instructions.push(Instr::Plus(1));
                }
            }
            Token::Dec => {
                if let Some(Instr::Plus(plus)) = instructions.last_mut() {
                    if plus.wrapping_sub(1) == 0 {
                        instructions.pop();
                    } else {
                        *plus = plus.wrapping_sub(1);
                    }
                } else {
                    instructions.push(Instr::Plus(255));
                }
            }
            Token::Bwd => {
                if let Some(Instr::Step(step)) = instructions.last_mut() {
                    if *step - 1 == 0 {
                        instructions.pop();
                    } else {
                        *step = *step - 1;
                    }
                } else {
                    instructions.push(Instr::Step(-1));
                }
            }
            Token::Fwd => {
                if let Some(Instr::Step(step)) = instructions.last_mut() {
                    if *step + 1 == 0 {
                        instructions.pop();
                    } else {
                        *step = *step + 1;
                    }
                } else {
                    instructions.push(Instr::Step(1));
                }
            }
            Token::Opn => {
                let opn = instructions.len();
                brackets.push_back(opn);
                instructions.push(Instr::Opn);
            }
            Token::Cls => {
                if let Some(opn) = brackets.pop_back() {
                    let cls = instructions.len();
                    mapping.insert(opn, cls);
                    mapping.insert(cls, opn);
                    instructions.push(Instr::Cls);
                } else {
                    return Err("unmathced ']'");
                }
            }
            Token::Get => {
                instructions.push(Instr::Get);
            }
            Token::Put => {
                instructions.push(Instr::Put);
            }
        }
    }
    if !brackets.is_empty() {
        return Err("unmathced '['");
    }
    Ok(Program {
        instructions,
        mapping,
    })
}

impl Program {
    pub fn run(&self, input: &Vec<u8>) -> Option<Vec<u8>> {
        let mut ptr = 0;
        let mut mem: Vec<u8> = vec![0];
        let mut ip = 0;
        let mut input_iter = input.into_iter();
        let mut output = Vec::new();
        while ip < self.instructions.len() {
            match self.instructions[ip] {
                Instr::Plus(plus) => {
                    mem[ptr] = mem[ptr].wrapping_add(plus);
                }
                Instr::Step(step) => {
                    if ptr as isize + step < 0 {
                        todo!("negative index");
                    }
                    ptr = (ptr as isize + step) as usize;
                    while ptr >= mem.len() {
                        mem.push(0);
                    }
                }
                Instr::Opn => {
                    if mem[ptr] == 0 {
                        ip = self.mapping[&ip];
                    }
                }
                Instr::Cls => {
                    if mem[ptr] != 0 {
                        ip = self.mapping[&ip];
                    }
                }
                Instr::Get => {
                    mem[ptr] = *input_iter.next().unwrap_or(&255);
                }
                Instr::Put => {
                    output.push(mem[ptr]);
                }
            }
            ip += 1;
        }
        Some(output)
    }
}
