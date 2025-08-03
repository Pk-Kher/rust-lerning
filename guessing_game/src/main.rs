use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    loop {
        println!("Guess the number!");
        let secret_number = rand::rng().random_range(1..=100);
        // println!("secret number is :{}", secret_number);
        loop {
            println!("Please input your guess.");

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Filed to read line");
            // let guess: u32 = guess.trim().parse().expect("Please enter a number");
             let guess: u32 =match guess.trim().parse() {
                Ok(num) => num,
                Err(_)=>continue,
            };
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Your number is less then the secret number"),
                Ordering::Greater => println!("Your number is greater then the secret number"),
                Ordering::Equal => {
                    println!("your number is equal to the secret number");
                    break;
                }
            }
            // println!("You guessed:{guess}",);
        }
        println!("If you want to exit press n other wise press any key and enter?");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Filed to read line");
        if answer.trim().to_lowercase() == "n" {
            break;
        }
    }
}
// 8 4 2 1
