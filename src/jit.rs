use crate::{Instr, Program};
use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};
use std::{
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
    mem, slice,
};

const TAPE_SIZE: usize = 30000;

macro_rules! my_dynasm {
    ($ops:ident $($t:tt)*) => {
        dynasm!($ops
            ; .arch x64
            ; .alias a_state, rcx
            ; .alias a_current, rdx
            ; .alias a_begin, r8
            ; .alias a_end, r9
            ; .alias retval, rax
            $($t)*
        )
    }
}

macro_rules! prologue {
    ($ops:ident) => {{
        let start = $ops.offset();
        my_dynasm!($ops
            ; sub rsp, 0x28
            ; mov [rsp + 0x30], rcx
            ; mov [rsp + 0x40], r8
            ; mov [rsp + 0x48], r9
        );
        start
    }};
}

macro_rules! epilogue {
    ($ops:ident, $e:expr) => {
        my_dynasm!($ops
            ; mov retval, $e
            ; add rsp, 0x28
            ; ret
        );
    };
}

macro_rules! call_extern {
    ($ops:ident, $addr:expr) => {
        my_dynasm!($ops
            ; mov [rsp + 0x38], rdx
            ; mov rax, QWORD $addr as _
            ; call rax
            ; mov rcx, [rsp + 0x30]
            ; mov rdx, [rsp + 0x38]
            ; mov r8,  [rsp + 0x40]
            ; mov r9,  [rsp + 0x48]
        );
    };
}

struct State<'a> {
    input: Box<dyn BufRead + 'a>,
    output: Box<dyn Write + 'a>,
    tape: [u8; TAPE_SIZE],
}

impl Program {
    pub fn run_with_jit(&self) -> Result<(), &str> {
        let mut ops = dynasmrt::x64::Assembler::new().unwrap();
        let mut loops = Vec::new();

        let start = prologue!(ops);

        for instr in self.instructions.iter() {
            match *instr {
                Instr::Plus(plus) => {
                    my_dynasm!(ops
                        ; add BYTE [a_current], plus as _
                    );
                }
                Instr::Step(step) => {
                    if step < 0 {
                        my_dynasm!(ops
                            ; sub a_current, -step as _
                            ; cmp a_current, a_begin
                            ; jb ->overflow
                        );
                    } else {
                        my_dynasm!(ops
                            ; add a_current, step as _
                            ; cmp a_current, a_end
                            ; jae ->overflow
                        );
                    }
                }
                Instr::Opn(_) => {
                    let backward_label = ops.new_dynamic_label();
                    let forward_label = ops.new_dynamic_label();
                    loops.push((backward_label, forward_label));
                    my_dynasm!(ops
                        ; cmp BYTE [a_current], 0
                        ; jz =>forward_label
                        ;=>backward_label
                    );
                }
                Instr::Cls(_) => {
                    let (backward_label, forward_label) = loops.pop().unwrap();
                    my_dynasm!(ops
                        ; cmp BYTE [a_current], 0
                        ; jnz =>backward_label
                        ;=>forward_label
                    );
                }
                Instr::Get => {
                    my_dynasm!(ops
                        ;; call_extern!(ops, State::getchar)
                        ; cmp al, 0
                        ; jnz ->io_failure
                    );
                }
                Instr::Put => {
                    my_dynasm!(ops
                        ;; call_extern!(ops, State::putchar)
                        ; cmp al, 0
                        ; jnz ->io_failure
                    );
                }
            }
        }
        assert!(loops.is_empty());

        my_dynasm!(ops
            ;; epilogue!(ops, 0)
            ;->overflow:
            ;; epilogue!(ops, 1)
            ;->io_failure:
            ;; epilogue!(ops, 2)
        );

        let buf = ops.finalize().unwrap();

        let f: extern "win64" fn(*mut State, *mut u8, *mut u8, *const u8) -> u8 =
            unsafe { mem::transmute(buf.ptr(start)) };
        let mut state = State::new(
            Box::new(BufReader::new(stdin())),
            Box::new(BufWriter::new(stdout())),
        );
        let start = state.tape.as_mut_ptr();
        let end = unsafe { start.offset(TAPE_SIZE as isize) };
        let res = f(&mut state, start, start, end);

        match res {
            0 => Ok(()),
            1 => Err("memory access error"),
            2 => Err("io error"),
            _ => panic!("unknown error"),
        }
    }
}

impl<'a> State<'a> {
    unsafe extern "win64" fn getchar(state: *mut State, cell: *mut u8) -> u8 {
        let state = &mut *state;
        let err = state.output.flush().is_err();
        (state
            .input
            .read_exact(slice::from_raw_parts_mut(cell, 1))
            .is_err()
            || err) as u8
    }

    unsafe extern "win64" fn putchar(state: *mut State, cell: *mut u8) -> u8 {
        let state = &mut *state;
        state
            .output
            .write_all(slice::from_raw_parts(cell, 1))
            .is_err() as u8
    }

    fn new(input: Box<dyn BufRead + 'a>, output: Box<dyn Write + 'a>) -> State<'a> {
        State {
            input,
            output,
            tape: [0; TAPE_SIZE],
        }
    }
}
