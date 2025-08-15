// #[derive(Debug)]
// enum IpAddrTypes{
//     V4,
//     V6
// }
// struct IpAddress{
//     kind:IpAddrTypes,
//     address:String
//
// }
// impl IpAddress{
//    fn define_ip(kind:IpAddrTypes,address:String)->Self{
//         Self{
//             kind,
//             address
//         }
//
//     }
//
// }
// #[derive(Debug)]
// enum IpAddrType {
//     V4(String),
//     V6(String),
// }
// #[derive(Debug)]
// enum Message {
// Quit,
// Move { x: i32, y: i32 },
// Write(String),
// ChangeColor(i32, i32, i32),
// }
// impl Message {
// fn show(){
//     println!("hllo");
//
// }

// fn call(&self) {
//     match self {
//         Message::Quit => println!("User quit"),
//         Message::Move { x, y } => println!("move x: {} y: {}", x, y),
//         Message::Write(msg) => println!("office IPv6 address: {}", msg),
//         Message::ChangeColor(r, g, b) => println!("R:{} G:{} B:{}", r, g, b),
//         // _=>println!(""),
//         // other=>println!("{}",other),
//         // other=>(), // nothing hapen unit tuple
//     }
// }
// }
// this is inbuild enum you don't need to define this;
// enum Option<T>{
//     Some(T),
//     None
// }
// struct UserData{
//     name:String,
//     age:Option<i32>  //it's works same as ? in the type script
// }

fn if_let_and_let_else_ex() {
    let x = 8;
    let y: Option<i32> = None;
    if let Some(data)=y{
        println!("{}",x+data);

    }else{
        println!("value is not define");
    }
    // let Some(_data) = y else {
    //     println!("value is not define");
    //     panic!("error");
    // };
    // println!("hello");
}

fn main() {
    // let home=IpAddress::define_ip(IpAddrTypes::V6,String::from("129.168.0.1"));
    if_let_jnd_let_else_ex();
    // let office=IpAddress::define_ip(IpAddrTypes::V4,String::from("129.168.0.1"));
    // println!("ip type:{:#?} address:{}",home.kind,home.address);
    // println!("ip type:{:#?} address:{}",office.kind,office.address);
    // let home = IpAddrType::V4(String::from("129.168.0.1"));
    // let office = IpAddrType::V6(String::from("::1"));
    // print_address(home);
    // print_address(office);
    // println!("{:#?} {:#?}", home, office);
    // Message::show();
    // let m2 = Message::Move { x: 30, y: 10 };
    // let m3 = Message::Write(String::from("hello"));
    // let m4 = Message::ChangeColor(4, 6, 8);
    //
    // m1.call();
    // m2.call();
    // m3.call();
    // m4.call();
    //
    // let user1=UserData{
    //     name:String::from("pk"),
    //     age:Some(32)
    // };
    // let user2=UserData{
    //     name:String::from("foo"),
    //     age:None
    // };
    //  ////////////
    // let if demo
    // let config_max = Some(3u8);
    // if let Some(value) = config_max {
    //     println!("{value}");
    // }
    // if let Some(max) = config_max {
    //     println!("{max}");
    // }else{
    //     println!("value is None");
    // }
    //  //////////
    // demo_let_else();
    // slice_demo();
    // let x: i8 = 5;
    // let y: Option<i8> = Some(2);
    // let sum=x+y; //error
    // let sum = match y {
    //     Some(v) => v + x,
    //     None => 0 + x,
    // };
    // println!("{}", sum);
    // struct_demo();
}
// fn print_address(add:IpAddrType){
//     match add{
//         IpAddrType::V4(address) => println!("office IPv4 address: {}", address),
//         IpAddrType::V6(address) => println!("office IPv6 address: {}", address),
//     }
//
// }
fn struct_demo() {
    #[derive(Debug)]
    struct Rect {
        width: i32,
        height: i32,
    }
    impl Rect {
        fn something(&self) -> String {
            let data = format!("{} X {}", self.width, self.height);
            String::from(data)
        }
    }
    let foo = Rect {
        width: 32,
        height: 32,
    };
    let data = Rect::something(&foo);
    println!("{:#?}", data);
    println!("{:#?}", foo.something());
}
fn _slice_demo() {
    let s = String::from("hello world");
    let a: &String = &s;
    let b: &str = a;
    // let array:[i32;5]=[3;5];
    // let data = &s[0..6];
    // let data= &array[0..2];
    println!("{:p}", a);
    println!("{:p}", b);
    // 0111 1111 1111 1110 1101 0011 1101 0100 1111 1111 0011 0000
}
fn _demo_let_else() {
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        // Penny,
        // Nickel,
        Dime,
        Quarter(UsState),
    }

    impl UsState {
        fn existed_in(&self, year: u32) -> bool {
            match self {
                UsState::Alaska => year >= 1898,
                UsState::Alabama => year >= 1999,
            }
        }
    }
    let data = Coin::Quarter(UsState::Alaska);
    let data1 = Coin::Quarter(UsState::Alabama);
    let data2 = Coin::Dime;
    println!("{:#?}", describe_state_quater(data));
    println!("{:#?}", describe_state_quater(data1));
    println!("{:#?}", describe_state_quater(data2));
    fn describe_state_quater(coin: Coin) -> Option<String> {
        // match coin{
        //     Coin::Quarter(value)=>Some(format!("{value:?}")),
        //     Coin::Dime=>None
        // }
        let Coin::Quarter(value) = coin else {
            return None;
        };
        if value.existed_in(1990) {
            Some(format!("{value:?} is pretty old "))
        } else {
            Some(format!("{value:?} new"))
        }
    }
}
