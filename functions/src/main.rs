fn main() {
    println!("Hello, world!");
    let mut value = 90;
    another_fn(&mut value);
    println!("{}", value);
    print_string_with_number(32, "i");
    let y = {
        let x = 5;
        x + 10
    };
    println!("The value of y is: {y}");
    println!("calling five function {}",five());

    let s=String::from("hello");
    change_string(s);
    // println!("{s}");
}
// fn dangling()->&String{
//     let s=String::from("hello");
//     &s
//
// }
fn change_string(mut s:String){
    println!("{s}");
    s.push_str(", world");
}
fn another_fn(x: &mut i32) {
    *x = 80;
    println!("hello from another func {}", x);
}

fn print_string_with_number(x: i32, y: &str) {
    println!("{x}{y}");
}
fn five()->i32{
    5
}
