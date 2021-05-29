use std::{
    collections::{BTreeMap, VecDeque},
    env, fs,
    io::{self, stdout, Read, Write},
};

struct Program {
    cmds: Vec<Cmd>,
    mapping: BTreeMap<usize, usize>,
}

enum Cmd {
    Inc, // +
    Dec, // -
    Bwd, // <
    Fwd, // >
    Opn, // [
    Cls, // ]
    Get, // ,
    Put, // .
}

fn compile(code: &String) -> Option<Program> {
    let mut cmds = Vec::new();
    let mut mapping = BTreeMap::new();
    let mut stack = VecDeque::new();
    for c in code.chars() {
        match c {
            '+' => cmds.push(Cmd::Inc),
            '-' => cmds.push(Cmd::Dec),
            '<' => cmds.push(Cmd::Bwd),
            '>' => cmds.push(Cmd::Fwd),
            '[' => {
                let opn = cmds.len();
                stack.push_back(opn);
                cmds.push(Cmd::Opn);
            }
            ']' => {
                if let Some(opn) = stack.pop_back() {
                    let cls = cmds.len();
                    mapping.insert(opn, cls);
                    mapping.insert(cls, opn);
                    cmds.push(Cmd::Cls);
                } else {
                    return None;
                }
            }
            ',' => cmds.push(Cmd::Get),
            '.' => cmds.push(Cmd::Put),
            _ => (),
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
                Cmd::Inc => {
                    mem[ptr] = mem[ptr].wrapping_add(1);
                }
                Cmd::Dec => {
                    mem[ptr] = mem[ptr].wrapping_sub(1);
                }
                Cmd::Bwd => {
                    if ptr == 0 {
                        todo!("negative index");
                    }
                    ptr -= 1;
                }
                Cmd::Fwd => {
                    ptr += 1;
                    if ptr == mem.len() {
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
