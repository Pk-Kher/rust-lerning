fn main() {
    //if condition
    // const CONDITION: bool = true;
    // let mut value: i32 = 89;
    // let num: i32 = return_value(CONDITION, &mut value);
    // println!("{num} {CONDITION} {value}");
    // /////////////////////////////////
    //
    // let mut name = String::from("pk");
    // strung_fn(&mut name);
    // println!("{name}");
    // /////////////////////
    // let res: String;
    // {
    //     res = loop_fun();
    // }
    // println!("{}", res);
    // let result;
    // {
    //     let s1 = String::from("Luffy");
    //     let s2 = String::from("Zoro");
    //
    //     result = longest(s1.as_str(), s2.as_str()); // ❌
    // }
    // println!("Result: {}", result);
    // loop_labeling();
    // while_loop();
    // for_loop();
    // let (x, y);
    // (x, ..) = (3, 4);
    // [.., y] = [1, 2];
    // // Fill the blank to make the code work
    // assert_eq!([x, y], [x, y]);
    // some_code();
    let mut num = String::new();
    println!("Please enter max number to generate fibbo");
    std::io::stdin()
        .read_line(&mut num)
        .expect("faild to read data");
    let num: i32 = num.trim().parse().expect("fail to convert into integer");
    fibo(num);
}
fn fibo(max: i32) {
    let mut data: i32 = 0;
    let mut i = 1;
    println!("your series is:");
    println!("{}", 0);
    loop {
        let tmp = data;
        data = data + i;
        println!("{}", data);
        i = tmp;
        if data+i >= max {
            break;
        }
    }
}
// 0
// 1
// 1
// 2
// 3
// 5
// fn some_code() {
//     // Integer addition
//     assert!(1u32 + 2 == 3);
//
//     // Integer subtraction
//     assert!(1i32 - 2 == -1);
//     assert!(1i8 - 2 == -1);
//     //
//     assert!(3 * 50 == 150);
//     // println!("{}",(9.6/3.2).abs());
//     // assert!(9.6 / 3.2 == 3.0); // error ! make it work
//     //
//     assert!(24 % 5 == 4);
//     // // Short-circuiting boolean logic
//     assert!(true && false == false);
//     assert!(true || false == true);
//     // assert!(!true == false);
//     // //
//     // // // Bitwise operations
//     println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
//     println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
//     println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
//     println!("1 << 5 is {}", 1u32 << 5); // 32
//     println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
// }
// assert!(a.round()==0.3);
// fn return_value(condition: bool, value: &mut i32) -> i32 {
//     *value = 90;
//     if condition { *value } else { 6 }
// }
// fn strung_fn(name: &mut String) {
//     println!("{name}");
//     *name = String::from("hello");
// }
//
// fn loop_fun<'a>() -> String {
//     let mut i: i32 = 0;
//     loop {
//         i += 1;
//         if i == 3 {
//             continue;
//         }
//         println!("hello from pk {i}");
//         if i == 5 {
//             // break String::from("end here");
//             return String::from("end");
//         }
//     }
// }
// fn longest(x: &str, y: &str) -> &'static str {
//     if x.len() > y.len() { "hello" } else { "hi" }
// }
// fn loop_labeling() -> String {
//     let mut i: i32 = 0;
//     'main_loop: loop {
//         i += 1;
//         if i == 9 {
//             continue 'main_loop;
//         }
//         println!("main loop {i}");
//         if i == 10 {
//             break 'main_loop String::from("main_loop");
//         }
//     }
// }
//
// fn while_loop() {
//     let a: [i32; 5] = [1, 2, 3, 4, 5];
//     let mut i: usize = 0;
//     while i < a.len() {
//         println!("print i:{i} and a[{i}]:{}", a[i]);
//         i += 1;
//     }
// }
//
// fn for_loop() {
//     let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
//     arr.reverse();
//     for element in arr {
//         println!("value: {element}");
//     }
//     for ele in (1..5).rev() {
//         println!("value {ele}");
//     }
// }
