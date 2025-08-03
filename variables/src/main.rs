use std::io;

fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x + 1;
        println!("x= {}", x);
    }
    // let spaces="   ";
    // let spaces:usize=spaces.len();
    // let data:u32="42".parse().expect("Could not parse");
    // let u_data=3.5;
    // let sum = 5 + 10;
    // let diff = 95.5 - 4.3;
    // let product = 4 * 30;
    // let div = 56.7 / 32.2;
    // let trunc = -5 / 3;
    // let rem = 43 % 5;
    // let t=false;
    // let c="asdad";
    //
    // tuple
    // let mut tup:(i32,f64,u8)=(500,-4.5,5);
    // let tup2=&mut tup;
    // println!("{}", tup.0);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let b:[i32;4]=[4;4];
    println!("please enter a index");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("fail to read data");
    let index:usize=index.trim().parse().expect("index enter was not a number");
    let element=a[index];
    println!("value in index {} is:{}",index,element);
}





