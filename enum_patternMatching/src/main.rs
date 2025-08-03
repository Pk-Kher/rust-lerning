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
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// impl Message {
//     fn call(&self) {
//         match self {
//             Message::Quit => println!("User quit"),
//             Message::Move { x, y } => println!("move x: {} y: {}", x, y),
//             Message::Write(msg) => println!("office IPv6 address: {}", msg),
//             Message::ChangeColor(r, g, b) => println!("R:{} G:{} B:{}", r, g, b),
//             // _=>println!(""),
//             // other=>println!("{}",other),
//             // other=>(), // nothing hapen unit tuple
//         }
//     }
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
fn main() {
    // let home=IpAddress::define_ip(IpAddrTypes::V6,String::from("129.168.0.1"));
    // let office=IpAddress::define_ip(IpAddrTypes::V4,String::from("129.168.0.1"));
    // println!("ip type:{:#?} address:{}",home.kind,home.address);
    // println!("ip type:{:#?} address:{}",office.kind,office.address);
    // let home = IpAddrType::V4(String::from("129.168.0.1"));
    // let office = IpAddrType::V6(String::from("::1"));
    // print_address(home);
    // print_address(office);
    // println!("{:#?} {:#?}", home, office);
    // let m1 = Message::Quit;
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
    demo_let_else()
}
// fn print_address(add:IpAddrType){
//     match add{
//         IpAddrType::V4(address) => println!("office IPv4 address: {}", address),
//         IpAddrType::V6(address) => println!("office IPv6 address: {}", address),
//     }
//
// }
fn demo_let_else() {
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
    let data=Coin::Quarter(UsState::Alaska);
    let data1=Coin::Quarter(UsState::Alabama);
    let data2=Coin::Dime;
    println!("{:#?}",describe_state_quater(data)); 
    println!("{:#?}",describe_state_quater(data1)); 
    println!("{:#?}",describe_state_quater(data2)); 
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
        }else{
            Some(format!("{value:?} new"))
        }
    }
}
