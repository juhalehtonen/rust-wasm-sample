fn main() {
    println!("Hello, WebAssembly");
}

// Functions that you wish to access from Javascript
// must be marked as no_mangle
#[no_mangle]
pub fn add_twenty_seven(n: i32) -> i32 {
    n + 27
}
