use std::io::{self, Write};
#[derive(Debug)]
struct User {
    name: String,
    email: String,
}
impl User {
    fn details(name: String, email: String) -> User {
        User { name, email }
    }
}
fn main() {
    //create empty vector
    user_functionality();
}
fn user_functionality() {
    let mut user_data: Vec<User> = Vec::new();
    loop {
        let mut user_input = String::new();
        println!("1. create user\n2. List users.\n3. Remove user\n4. Exit");
        print!("Select Option: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read user input");
        let user_input = user_input.trim();
        match user_input {
            "1" => create_user(&mut user_data),
            "2" => list_user(&user_data),
            "3" => remove_user(&mut user_data),
            "4" => break,
            _ => println!("please select proper option"),
        }
    }
}

fn create_user(user_details: &mut Vec<User>) {
    let mut name = String::new();
    let mut email = String::new();
    print!("Enter User name:");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut name)
        .expect("failed to read user name");
    print!("Enter User email:");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut email)
        .expect("failed to read user email");
    let data = User::details(
        name.trim().to_string(),
        email.trim().to_string(),
    );
    user_details.push(data);
    list_user(user_details);
}

fn list_user(user_details: &Vec<User>) {
    user_table_header("Users");
    for (index, user) in user_details.iter().enumerate() {
        println!("|{:<10}|{:<15}|{:<25}|", index + 1, user.name, user.email);
    }
    if user_details.len() == 0 {
        println!("|{:^52}|", "User is not available");
    }
    println!("|{:-<10}+{:-<15}+{:-<25}|", "", "", "");
}

fn remove_user(user_details: &mut Vec<User>) {
    let mut id = String::new();
    print!("please enter user id:");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut id)
        .expect("Failed to read user id");
    let id: usize = id.trim().parse().expect("please provide valid index");
    let user = user_details.swap_remove(id-1);
    user_table_header("Remove User");
    println!("|{:<10}|{:<15}|{:<25}|", id, user.name, user.email);
    list_user(user_details);
}
fn user_table_header(header_title: &str) {
    println!("");
    println!("|{:^52}|", header_title);
    println!("|{:-<10}+{:-<15}+{:-<25}|", "", "", "");
    println!("|{:<10}|{:<15}|{:<25}|", "id", "name", "email");
    println!("|{:-<10}+{:-<15}+{:-<25}|", "", "", "");
}
fn _vertor_demo_test() {
    //https://doc.rust-lang.org/nomicon/vec/vec.html
    let v: Vec<i32> = Vec::new();
    let mut v1 = vec![8, 1, 2, 3, 4];
    let string_v = vec![String::from("hello")];
    v1.push(5);
    v1.pop();
    v1.push(6);
    v1.sort(); //it's mutate value
    // v1.clear();
    let third = &v1[2]; // to will get copy not move
    //let string_vec_value=string_v[0]; ///it will give you error you can not perform move
    let string_vec_value = &string_v[0]; //it will give you error
    println!(
        "string_v-> {:#?} string_vec_value->{}",
        string_v, string_vec_value
    ); // so you can do this
    println!("v1[2]-> {:#?} ", v1[2]); // so you can do this
    println!("third->{third:#?}");
    println!("v->{v:#?}\nv1-> {v1:#?}");

    let get_0 = v1.get(0); //Option<T>;
    match get_0 {
        Some(data) => println!("{data}"),
        None => println!("data is not available at 0"),
    }
    // let mut v = vec![1, 2, 3, 4, 5];
    //
    // let first = &v[0];

    // v.push(6); //error
    for i in &v1 {
        //use &v1 other wise move will occre
        println!("i->{i}");
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{v:#?}");
    // let mut users: Vec<User> = Vec::new();

    // users.push(User {
    //     name: String::from("pk"),
    //     email: String::from("pk@gmail.com"),
    // });
    // users.push(User::details(
    //     String::from("name"),
    //     String::from("email@gmail.com"),
    // ));
    // println!("{:#?}", users);
    // for i in &users {
    //     println!("name:{}email:{}", i.name, i.email);
    // }
}
