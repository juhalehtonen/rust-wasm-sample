fn main() {
    println!("Hello, WebAssembly!");
}

// Functions that you wish to access from Javascript
// must be marked as no_mangle
#[no_mangle]
pub fn add_two_numbers(x: i32, y: i32) -> i32 {
    x + y
}
