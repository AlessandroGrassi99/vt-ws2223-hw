use criterion::{criterion_group, criterion_main, Criterion};
use interpreter::{InstructionCycle, Interpreter, Program};

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

fn load_program(size: i32, mut prob: [i32; 5]) -> (Program, i32, i32) {
    let mut buf: [u8; 50000] = [0; 50000];
    let mut register_a: i32 = 0;
    let mut register_l: i32 = 0;
    let seed: i32 = 1;

    unsafe {
        init(
            buf.as_mut_ptr() as *mut i8,
            size,
            prob.as_mut_ptr(),
            seed,
            &mut register_a,
            &mut register_l,
        );
    }

    (Program::new(buf, size), register_a, register_l)
}

fn scenario_1(c: &mut Criterion) {
    let size: i32 = 10000;
    let prob: [i32; 5] = [0, 1, 0, 0, 0];
    let (program, register_a, register_l) = load_program(size, prob);
    let mut interpreter = Interpreter::new(program, register_a, register_l);

    c.bench_function("scenario 1", |b| {
        b.iter(|| {
            interpreter.reset(register_a, register_l);
            while !interpreter.halted() {
                interpreter.step();
            }
        })
    });
}

fn scenario_2(c: &mut Criterion) {
    let size: i32 = 10000;
    let prob: [i32; 5] = [1, 1, 1, 0, 0];
    let (program, register_a, register_l) = load_program(size, prob);
    let mut interpreter = Interpreter::new(program, register_a, register_l);

    c.bench_function("scenario 2", |b| {
        b.iter(|| {
            interpreter.reset(register_a, register_l);
            while !interpreter.halted() {
                interpreter.step();
            }
        })
    });
}

fn scenario_3(c: &mut Criterion) {
    let size: i32 = 10000;
    let prob: [i32; 5] = [1, 9, 1, 5, 5];
    let (program, register_a, register_l) = load_program(size, prob);
    let mut interpreter = Interpreter::new(program, register_a, register_l);

    c.bench_function("scenario 3", |b| {
        b.iter(|| {
            interpreter.reset(register_a, register_l);
            while !interpreter.halted() {
                interpreter.step();
            }
        })
    });
}

fn scenario_4(c: &mut Criterion) {
    let size: i32 = 50000;
    let prob: [i32; 5] = [1, 9, 1, 5, 5];
    let (program, register_a, register_l) = load_program(size, prob);
    let mut interpreter = Interpreter::new(program, register_a, register_l);

    c.bench_function("scenario 4", |b| {
        b.iter(|| {
            interpreter.reset(register_a, register_l);
            while !interpreter.halted() {
                interpreter.step();
            }
        })
    });
}

criterion_group!(benches, scenario_1, scenario_2, scenario_3, scenario_4);
criterion_main!(benches);
