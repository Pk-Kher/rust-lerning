fn main() {
    let s = String::from("hello");
    // let hello = &s[0..5];
    // let word=&s[6..11];
    // let i = &s;
    first_word(&s);
    // let data:Vec<String>=s.split_whitespace().map(|s| s.to_string()).collect();
    // println!("{:?}",data);
    // for item in s.split(" ") {
    //     println!("{}",item);
    // }
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &s[3..];
    println!("{:#?}", slice);
    // clear_string(&mut s);
}
// fn clear_string(s: &mut String) {
//     s.clear();
// }
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        let data = b' ';
        println!("{} {} {}", i, item, data);
        if item == data {
            return &s[0..i];
        }
    }
    &s[..]
}
