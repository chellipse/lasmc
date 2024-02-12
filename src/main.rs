use dynasm::dynasm;
use dynasmrt::{ExecutableBuffer, DynasmApi, DynasmLabelApi};

use std::{io, slice, mem};
use std::io::Write;

mod parse;
use parse::parse;

mod read_file;
use read_file::read_input_file;

pub extern "sysv64" fn print(buffer: *const u8, length: u64) -> bool {
    io::stdout()
        .write_all(unsafe { slice::from_raw_parts(buffer, length as usize) })
        .is_ok()
}

fn main() {
    let mut ops = dynasmrt::x64::Assembler::new().unwrap();
    let string = "Hello World!\n";

    dynasm!(ops
        ; .arch x64
        ; ->hello:
        ; .bytes string.as_bytes()
    );
    let one = ops.offset();
    // dbg!(one);

    dynasm!(ops
        ; .arch x64

        ; lea rdi, [->hello]
        ; xor esi, esi
        ; mov sil, BYTE string.len() as _
        ; mov rax, QWORD print as _
        ; sub rsp, BYTE 0x28
        ; call rax
        ; add rsp, BYTE 0x28

        ; ret
    );
    // let two = ops.offset();
    // dbg!(two);

    let buf: ExecutableBuffer = ops.finalize().unwrap();

    let code: extern "C" fn() = unsafe { mem::transmute(buf.ptr(one)) };

    // println!("Started.");

    code(); // This will execute the `ret` instruction.

    // println!("\nFinished.");

    let input = read_input_file();
    parse(input)
}

