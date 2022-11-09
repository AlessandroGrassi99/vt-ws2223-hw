use interpreter::{InstructionCycle, Interpreter, Program};
use std::time::Instant;

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

const WARM_UP_CYCLE: usize = 100;
const BATCH_SIZE: usize = 1000;

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
        println!("Running scenario {}...", i + 1);
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
        for _ in 0..BATCH_SIZE {
            while !interpreter.halted() {
                interpreter.step();
            }
        }

        let mut clocks: usize = 0;
        let now = Instant::now();
        for _ in 0..BATCH_SIZE {
            interpreter.reset(register_a, register_l);
            while !interpreter.halted() {
                interpreter.step();
                clocks += 1;
            }
        }

        let duration = now.elapsed();
        let ns = duration.as_nanos() as f64;

        println!(
            "Completed scenario {} with a batch size of {}:\n\
            \tTime required: {}\n\
            \tTotal Clock cycles: {}\n\
            \tClock cycles per sec: {}\n\
            \tAvg time per clock cycle: {}",
            i + 1,
            BATCH_SIZE,
            format_time(ns / BATCH_SIZE as f64),
            clocks / BATCH_SIZE,
            (1e9f64 / ns) * clocks as f64,
            format_time(ns / clocks as f64)
        );
    }
}

fn format_time(ns: f64) -> String {
    if ns < 1.0 {
        format!("{:>6} ps", ns * 1e3)
    } else if ns < 10f64.powi(3) {
        format!("{:>6} ns", ns)
    } else if ns < 10f64.powi(6) {
        format!("{:>6} us", ns / 1e3)
    } else if ns < 10f64.powi(9) {
        format!("{:>6} ms", ns / 1e6)
    } else {
        format!("{:>6} s", ns / 1e9)
    }
}
