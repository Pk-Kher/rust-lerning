fn main() {
    let mut s1=String::from("hello");
    // let s2=s1.clone();
    // let length=cal_length(&s1);
    // change(&mut s1);
    let r1=&s1;
    println!("{}",r1);
    let r2=&s1;
    let r3=&mut s1;
    println!("{}",r3);
}


fn change(s:&mut String){
    s.push_str(", dear");

}
// fn cal_length(s:&String)->usize{
//     s.len()
// }
