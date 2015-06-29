#[no_mangle]
pub extern fn increment(mut input: i32) -> i32 {
    for _ in 0..100000000 {
        input = input + 1;
    }
    input
}
