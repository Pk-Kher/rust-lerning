use std::fmt::Display;

use orphan_rule::{Pirate, PrirateHi};
use summary_traits::{NewsArtical, SocialPost, notify};

use crate::life_time::{foo_life_time, life_time_ex};
use crate::summary_traits::{Summary, notify_v2, notify_v3, notify_v4, return_summary_social};

pub mod life_time;
pub mod orphan_rule;
pub mod summary_traits;

fn main() {
    // calling_notify_with_two_impl_traits();
    // let social = return_summary_social();
    // println!("{}", social.summarize());
    // max_num_from_pair();
    // life_time_ex();
    // foo_life_time();
    questions3();
}

fn questions3() {
    #[derive(Debug)]
    struct TestResult {
        scores: Vec<usize>,
        curve: Option<usize>,
    }

    impl TestResult {
        fn get_curve(&self) -> &Option<usize> {
            &self.curve
        }
        fn apply_curve(&mut self) {
            if let Some(curve) = self.get_curve() {
                // if let Some(curve) = self.curve {
                for data in self.scores.clone().iter_mut() {
                // for data in self.scores.iter_mut() {
                    *data += curve;
                }
            }
        }
    }

    // let mut result = TestResult {
    //     score: vec![10, 20, 30],
    //     curve: Some(1),
    // };
    // println!("curve {:#?}", result.get_curve());
    // result.apply_curve();
    // println!("curve {:#?}", result);

    let mut result = TestResult {
        scores: vec![20, 50, 30],
        curve: Some(10),
    };
    result.apply_curve();
    println!("{:?}", result.scores);



    let name: Option<String> = Some("Nami".to_string());

    if let Some(n) = &name {
        // Borrow inner value
        println!("Name: {}", n);
    }
    let data = &name;
    let n = name.as_ref().map(|s| s.len());
    println!("{:?}", n);
}
// fn _max_num_from_pair() {
//     struct Pair<T> {
//         x: T,
//         y: T,
//     }
//     impl<T> Pair<T> {
//         fn new(x: T, y: T) -> Self {
//             Self { x, y }
//         }
//     }
//     impl<T: Display + PartialOrd> Pair<T> {
//         fn cmp_display(&self) {
//             if self.x >= self.y {
//                 println!("x is greater or eq to y");
//             } else {
//                 println!("y is greater to x");
//             }
//         }
//     }
//
//     let data=Pair::new(40,60);
//     let data1=Pair::new('q','d');
//     data.cmp_display();
//     data1.cmp_display();
// }
// // notify version 4 have tow traits implementation required so pirated have both Display and
// // Summary
// impl Summary for Pirate {
//     fn get_author(&self) -> String {
//         format!("{}", self.name)
//     }
// }
// fn calling_notify_with_two_impl_traits() {
//     let pirate = Pirate::new(String::from("zoro"), 89898989);
//     notify_v4(&pirate);
// }
fn _orphan_rule_test() {
    let pirate = Pirate::new(String::from("luffy"), 9999999);
    println!("{}", pirate);
    let luffy_hi = String::from("luffy");
    println!("{}", luffy_hi.say_hi());
}
fn traits_basic_uses() {
    //trait use
    let artical = NewsArtical {
        headline: String::from("News Head line"),
        author: String::from("pk"),
        content: Some(String::from("News Content")),
        location: String::from("None"),
    };
    let social_post = SocialPost {
        username: String::from("Author"),
        content: String::from("Social opst content"),
        repost: String::from("no"),
        reply: String::from("no reply"),
    };
    let artical2 = NewsArtical::new(
        String::from("head line for artical2"),
        String::from("location 2"),
        String::from("some Author"),
        Some(String::from("content 2")),
    );
    notify(&artical);
    notify(&artical2);
    notify(&social_post);
    notify_v2(&social_post);
    notify_v3(&artical, &artical2);
    // println!(
    //     "Social Post:{} author:{}",
    //     social_post.summarize(),
    //     social_post.get_author()
    // );
    // println!("News Artical:{}", artical2.summarize());
    // println!("News Artical:{}", artical.summarize());
}
fn _generic_mix() {
    #[derive(Debug)]
    struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }
    impl<X1, Y1> Point<X1, Y1> {
        fn mix_two_types<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point { x: self.x, y: other.y }
        }
    }
    let p1 = Point { x: 12, y: 20.5 };
    let p2 = Point { x: "hello", y: 'c' };

    let p3 = p1.mix_two_types(p2);
    // p1;p2; // p1,p2 is move so you can not use here
    println!("{:?}", p3);
}
// fn generic_with_struct(){
//     #[derive(Debug)]
//     struct Point<T>{
//         x:T,
//         y:T
//     }
//     impl<T> Point<T>{
//         fn x(&self)->&T{
//             &self.x
//         }
//
//     }
//     impl Point<f32>{
//         fn distance_from_origin(&self)->&f32{
//            &self.y
//         }
//
//     }
//     let data=Point{x:12,y:23};
//     let data2=Point{x:34.3,y:45.3};
//
//     println!("{:#?}",data.x());
//     println!("{:#?}",data2.distance_from_origin());
//
// }

///generic example
// fn find_larget_using_generic<T>(data: &[T]) -> &T {
//     let mut largest = &data[0];
//     for i in data {
//         if i > largest { // this will give you error
//             largest = i;
//         }
//     }
//     largest
// }
fn _find_largest_num(numbers_list: &[i32]) -> &i32 {
    // let numbers_list = vec![1, 2, 3, 4, 5];
    let mut largest_num = &numbers_list[0];
    for i in numbers_list {
        if i > largest_num {
            largest_num = i;
        }
    }
    largest_num
}

fn _find_largest_char(char_arr: Vec<char>) -> char {
    let mut large = char_arr[0];
    for i in &char_arr {
        if *i > large {
            large = *i;
        }
    }
    large
}
