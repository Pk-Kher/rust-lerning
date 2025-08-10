unsafe extern "C" {
    fn add_numbers(a: i32, b: i32,c:i32) -> i32;
}

fn main() {
    unsafe {
        let result = add_numbers(5, 7,10);
        println!("Result: {result}");
    }
}
