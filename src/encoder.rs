use dynasm::dynasm;
#[allow(unused_imports)] // these are all needed, trust
use dynasmrt::{ExecutableBuffer, DynasmApi, DynasmLabelApi};
use dynasmrt::x64;

use std::{io, slice, mem};
use std::io::Write;

use lazy_static::lazy_static;
use std::collections::HashMap;

// local module
use crate::parser::Expression;

static STRING: &str = "Hello World!";

/// Operator implementations
fn quote() {
    println!("This is the quote function.");
}

// Define the type alias
type FnMap = HashMap<String, fn()>;
// Define global HashMap of operator implementations
lazy_static! {
    static ref PRIME_OP_MAP: FnMap = {
        let mut m = FnMap::new();
        m.insert(String::from("quote"), quote);
        m
    };
}

fn recursive_encode(ops: &mut x64::Assembler, list: &Vec<Expression>) {
    if &list.len() < &1 { todo!() } // should return Nil
    match &list[0] {
        Expression::List(l) => {
            recursive_encode(ops, &l);
        },
        Expression::Atom(s) => {
            if let Some(op_impl) = PRIME_OP_MAP.get(s) {
                // dbg!(op_impl);
                op_impl();
            }
        },
    }
}

// create my_dynasm! as a macro for dynasm! w/ x64 arch
macro_rules! my_dynasm {
    ($ops:ident $($t:tt)*) => {
        dynasm!($ops
            ; .arch x64
            $($t)*
        )
    }
}

macro_rules! setup {
    ($ops:ident) => {my_dynasm!($ops
        ; push r12
        ; push r13
        ; push r14
        ; push r15
    );};
}

macro_rules! clean_up {
    ($ops:ident) => {my_dynasm!($ops
        ; pop r12
        ; pop r13
        ; pop r14
        ; pop r15
    );};
}

// create malloc! macro which will allocate <usize> and return a pointer into Rax
// depends on my_dynasm!
macro_rules! malloc {
    ($ops:ident, $usize:expr) => {my_dynasm!($ops
        ; mov rdi, $usize
        ; mov rax, QWORD libc::malloc as _
        ; call rax
    );};
}

macro_rules! show_reg {
    ($ops:ident, $reg:ident) => {my_dynasm!($ops
        ; mov rdi, $reg
        ; mov rax, QWORD print_hex as _
        ; call rax
    );};
}

pub fn encode(cst: Vec<Expression>) -> (ExecutableBuffer, extern "C" fn()) {
    let mut ops = x64::Assembler::new().unwrap();

    for e in cst.iter() {
        match e {
            Expression::List(l) => {
                recursive_encode(&mut ops, l);
            },
            Expression::Atom(s) => {
                println!("Ignoring stray Atom: {}", s);
            },
        }
    }

    my_dynasm!(ops
        ; ->hello:
        ; .bytes STRING.as_bytes()
        ; ->fn_hello:
        ; lea rdi, [->hello]
        ; xor esi, esi
        ; mov sil, BYTE STRING.len() as _
        ; mov rax, QWORD print as _
        ; call rax
        ; ret
    );

    let entry_ptr = ops.offset();

    my_dynasm!(ops
    );

    my_dynasm!(ops
        // 0 = value, 8 = link list ptr, 16 = pointer to last element
        ; push r12
        ; push r13
        ;; malloc!(ops, 24)
        ; mov QWORD [rax], 0xffff as _
        ; mov [rax+16], rax

        ; mov r12, rax
        ;; malloc!(ops, 16)
        ; mov QWORD [r12+8], rax

        ; mov r13, [r12]
        ; mov r14, [r12+8]
        ;; show_reg!(ops, r13)
        ;; show_reg!(ops, r14)
        ; pop r12
        ; pop r13


        // ; mov rax, ->fn_hello
        ; lea rax, [->fn_hello]
        // ;; show_reg!(ops, rax)
        ; call rax
        // ;; show_reg!(ops, rax)
    );

    my_dynasm!(ops
        ; ret
    );


    let buf: ExecutableBuffer = ops.finalize().unwrap();

    let code: extern "C" fn() = unsafe { mem::transmute(buf.ptr(entry_ptr)) };

    (buf, code)
}


#[allow(dead_code)]
pub extern "sysv64" fn print(buffer: *const u8, length: u64) -> bool {
    io::stdout()
        .write_all(unsafe { slice::from_raw_parts(buffer, length as usize) })
        .is_ok()
}

#[allow(dead_code)]
pub extern "sysv64" fn print_hex(value: u64) -> bool {
    println!("{:x}", value);
    true
}

