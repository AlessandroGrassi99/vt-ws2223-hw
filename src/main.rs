mod interpreter;
mod program;

use interpreter::{InstructionCycle, Interpreter};
use program::Program;

extern "C" {
    pub fn init(
        buf: *mut ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        prob: *mut ::std::os::raw::c_int,
        seed: ::std::os::raw::c_int,
        rA: *mut ::std::os::raw::c_int,
        rL: *mut ::std::os::raw::c_int,
    );
}

fn main() {
    let mut buf: [u8; 50000] = [0; 50000];
    let mut register_a: i32 = 0;
    let mut register_l: i32 = 0;

    let mut scenario: [(i32, [i32; 5], i32); 4] = [
        (10000, [0, 1, 0, 0, 0], 1),
        (10000, [1, 1, 1, 0, 0], 1),
        (10000, [1, 9, 1, 5, 5], 1),
        (50000, [1, 9, 1, 5, 5], 1),
    ];

    for (i, item) in scenario.iter_mut().enumerate() {
        println!("Scenario {}", i + 1);
        unsafe {
            init(
                buf.as_mut_ptr() as *mut i8,
                item.0,
                item.1.as_mut_ptr(),
                item.2,
                &mut register_a,
                &mut register_l,
            );
        }

        let program = Program::new(buf, item.0);
        let mut interpreter = Interpreter::new(program, register_a, register_l);

        while !interpreter.halted() {
            interpreter.step();
        }
    }
}
