use std::{
    collections::{BTreeMap, VecDeque},
    env, fs,
    io::{self, stdout, Read, Write},
};

struct Program {
    cmds: Vec<Cmd>,
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

enum Cmd {
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

fn compile(code: &String) -> Option<Program> {
    let tokens = tokenize(&code);
    let mut cmds = Vec::new();
    let mut mapping = BTreeMap::new();
    let mut stack = VecDeque::new();
    for t in tokens.iter() {
        match t {
            Token::Inc => {
                if let Some(Cmd::Plus(plus)) = cmds.last_mut() {
                    if plus.wrapping_add(1) == 0 {
                        cmds.pop();
                    } else {
                        *plus = plus.wrapping_add(1);
                    }
                } else {
                    cmds.push(Cmd::Plus(1));
                }
            }
            Token::Dec => {
                if let Some(Cmd::Plus(plus)) = cmds.last_mut() {
                    if plus.wrapping_sub(1) == 0 {
                        cmds.pop();
                    } else {
                        *plus = plus.wrapping_sub(1);
                    }
                } else {
                    cmds.push(Cmd::Plus(255));
                }
            }
            Token::Bwd => {
                if let Some(Cmd::Step(step)) = cmds.last_mut() {
                    if *step - 1 == 0 {
                        cmds.pop();
                    } else {
                        *step = *step - 1;
                    }
                } else {
                    cmds.push(Cmd::Step(-1));
                }
            }
            Token::Fwd => {
                if let Some(Cmd::Step(step)) = cmds.last_mut() {
                    if *step + 1 == 0 {
                        cmds.pop();
                    } else {
                        *step = *step + 1;
                    }
                } else {
                    cmds.push(Cmd::Step(1));
                }
            }
            Token::Opn => {
                let opn = cmds.len();
                stack.push_back(opn);
                cmds.push(Cmd::Opn);
            }
            Token::Cls => {
                if let Some(opn) = stack.pop_back() {
                    let cls = cmds.len();
                    mapping.insert(opn, cls);
                    mapping.insert(cls, opn);
                    cmds.push(Cmd::Cls);
                } else {
                    return None;
                }
            }
            Token::Get => {
                cmds.push(Cmd::Get);
            }
            Token::Put => {
                cmds.push(Cmd::Put);
            }
        }
    }
    Some(Program { cmds, mapping })
}

impl Program {
    fn run(&self, input: &Vec<u8>) -> Option<Vec<u8>> {
        let mut ptr = 0;
        let mut mem: Vec<u8> = vec![0];
        let mut ip = 0;
        let mut input_iter = input.into_iter();
        let mut output = Vec::new();
        while ip < self.cmds.len() {
            match self.cmds[ip] {
                Cmd::Plus(plus) => {
                    mem[ptr] = mem[ptr].wrapping_add(plus);
                }
                Cmd::Step(step) => {
                    if ptr as isize + step < 0 {
                        todo!("negative index");
                    }
                    ptr = (ptr as isize + step) as usize;
                    while ptr >= mem.len() {
                        mem.push(0);
                    }
                }
                Cmd::Opn => {
                    if mem[ptr] == 0 {
                        ip = self.mapping[&ip];
                    }
                }
                Cmd::Cls => {
                    if mem[ptr] != 0 {
                        ip = self.mapping[&ip];
                    }
                }
                Cmd::Get => {
                    mem[ptr] = *input_iter.next().unwrap_or(&255);
                }
                Cmd::Put => {
                    output.push(mem[ptr]);
                }
            }
            ip += 1;
        }
        Some(output)
    }
}

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();
    if let Some(filename) = args.get(1) {
        let code = fs::read_to_string(filename).unwrap_or_else(|_| {
            panic!("cannnot read file `{}'", filename);
        });
        let program = compile(&code).unwrap();
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input)?;
        let output = program.run(&input).unwrap();
        stdout().write_all(&output)?;
    } else {
        eprintln!("filename not specified");
    }
    Ok(())
}
