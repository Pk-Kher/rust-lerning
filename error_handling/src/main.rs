use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};


// struct Guess{
//     value:i32,
// }
// impl Guess{
//     fn new(value:i32)->Guess{
//         if value <1 || value >100{
//             panic!("not valid value");
//         }
//         Guess{
//             value
//         }
//     }
//
// }

fn main() -> Result<(), Box<dyn Error>> {
    // let value=Guess::new(3);
    //Result<T,E>
    // let file_result = File::open("hello.txt");
    // let mut _file = match file_result {
    //     Ok(file) => file,
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(file) => file,
    //             Err(e) => panic!("There is some problem while opening file {e:?}"),
    //         },
    //         _ => panic!("There is some problem {e:?}"),
    //     },
    // };
    // same using the unwrap_or_else
    // let file_result = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("error while creating file {error:?}");
    //         })
    //     } else {
    //         panic!("problem while opening file {error:?}");
    //     }
    // });
    //  expect and unwrap to handle error
    // let file=File::open("helo.txt").unwrap();
    // let file2=File::open("helo.txt").expect("Field to open file");
    match read_username_1() {
        Ok(data) => println!("{data:}"),
        Err(e) => println!("{e:#?}"),
    }
    match read_file_use_question() {
        Ok(data) => println!("{data:}"),
        Err(e) => println!("{e:#?}"),
    }
    match read_file_inbuild_lib() {
        Ok(data) => println!("{data:}"),
        Err(e) => println!("{e:#?}"),
    }
    match read_file_short_ver() {
        Ok(data) => println!("{data:}"),
        Err(e) => println!("{e:#?}"),
    }
    question_op_with_option("hello");
    //try ? op in the main function for that we need to chnage type of the return function of the
    //main
    let _file_buf = File::open("hello.txt")?;
    Ok(())
}
fn question_op_with_option(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn read_file_inbuild_lib() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn read_file_short_ver() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)  ?;
    Ok(username)
}
fn read_file_use_question() -> Result<String, io::Error> {
    let mut file_buffer = File::open("hello.txt")?;
    let mut username = String::new();
    file_buffer.read_to_string(&mut username)?;
    return Ok(username);
}

fn read_username_1() -> Result<String, io::Error> {
    let file_buffer = File::open("hello.txt");
    let mut file = match file_buffer {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create_new("hello.txt") {
                Ok(file) => file,
                Err(e) => return Err(e),
            },
            _ => return Err(e),
        },
    };
    let mut user_name = String::new();
    match file.read_to_string(&mut user_name) {
        Ok(_) => Ok(user_name),
        Err(e) => Err(e),
    }
}
