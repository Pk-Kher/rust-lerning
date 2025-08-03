#[derive(Debug)]
struct User {
    name: String,
    is_active: bool,
    email: String,
    sign_in_count: u32,
}
//tuple struct
#[derive(Debug)]
struct Color(i32, i32, i32);

fn main() {
    use_struct_diff_ways();
}
fn use_struct_diff_ways() {
    let mut user = User {
        name: String::from("pk"),
        email: String::from("pk@gmail.om"),
        is_active: true,
        sign_in_count: 32,
    };
    let black = Color(0, 0, 0);
    //desctucture
    let Color(_x, _y, _z) = black;
    user.email = String::from("pk1@gmail.com");

    let _user2 = user_bulder(String::from("foo@gmail.com"), String::from("name"));
    println!(
        "user1 user2 {:#?} {:#?} {:#?} {:#?}",
        user.sign_in_count, user.is_active, user.email, user.name
    );
    let _user3 = User {
        email: String::from("foo@gmail.com"),
        ..user
    };
    let user4 = User {
        name: String::from("pk"),
        email: String::from("pk@gmail.om"),
        is_active: true,
        sign_in_count: 32,
    };
    let _user5 = User {
        name: String::from("ho"),
        email: String::from("foo1@gmail.com"),
        ..user4
    };
    println!("use can still use user 3{:#?}", user4.is_active);
}
fn user_bulder(email: String, name: String) -> User {
    User {
        name,
        email,
        is_active: false,
        sign_in_count: 12,
    }
}
