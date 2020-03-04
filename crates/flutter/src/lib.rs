#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
extern "C" {}
struct Hello;

impl Hello {
    fn echo() {
        println!("hello");
    }
}
