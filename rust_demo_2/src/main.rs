extern crate libc;

// Run c from Rust as efficiently as native.
extern {
    fn add_one(input: libc::c_int) -> libc::c_int;
}

fn main() {
    let input = 4;
    let output = unsafe { add_one(input) };
    println!("{}", output);
}
