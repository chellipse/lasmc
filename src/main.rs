use dynasm::dynasm;
use dynasmrt::{DynasmApi, DynasmLabelApi, ExecutableBuffer};

fn main() {
    let mut ops = dynasmrt::x64::Assembler::new().unwrap();

    dynasm!(ops
        ; ret
    );

    let buf: ExecutableBuffer = ops.finalize().unwrap();

    let code: extern "C" fn() = unsafe { std::mem::transmute(buf.ptr()) };

    code(); // This will execute the `ret` instruction.
}
