fn main() {
    println!("Hello, WebAssembly!");
}

// Do not apply the standard name mangling. Set the symbol for this item to its identifier.
// The no_mangle attribute turns off Rust's name mangling, so that it is easier to link to.
// Functions that you wish to access from Javascript must be marked as no_mangle.
#[no_mangle]
pub fn add_two_numbers(x: i32, y: i32) -> i32 {
    x + y
}
