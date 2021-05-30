use bf::compile;
use std::{
    env, fs,
    io::{self, stdout, Read, Write},
};

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
