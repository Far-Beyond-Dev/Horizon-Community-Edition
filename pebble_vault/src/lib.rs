use std::os::raw::c_int;

extern "C" {
    fn Add(a: c_int, b: c_int) -> c_int;
}

fn main() {
    let a: c_int = 5;
    let b: c_int = 7;
    let result: c_int;

    unsafe {
        result = Add(a, b);
    }

    println!("The sum of {} and {} is {}", a, b, result);
}

pub fn gotest() {
    main();
}