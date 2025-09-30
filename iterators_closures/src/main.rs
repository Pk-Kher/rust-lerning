use std::thread;

// #[derive(PartialEq)]
#[derive(Debug)]
enum Color {
    Red,
    Blue,
}
#[derive(Debug)]
struct Inventory {
    shirt: Vec<Color>,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory { shirt: vec![] }
    }
    fn add_random_data(&mut self, count: i32) {
        for i in 0..count {
            if i % 2 == 0 {
                self.shirt.push(Color::Blue);
            } else {
                self.shirt.push(Color::Red);
            }
        }
    }
    fn gift(&self, choice: Option<Color>) -> Color {
        choice.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> Color {
        let mut red_short_count = 0;
        let mut blue_short_count = 0;
        for data in &self.shirt {
            match data {
                Color::Red => red_short_count += 1,
                Color::Blue => blue_short_count += 1,
            }
        }
        if red_short_count > blue_short_count {
            Color::Red
        } else {
            Color::Blue
        }
    }
}

fn main() {
    let mut inventory1 = Inventory::new();
    inventory1.add_random_data(5);
    let my_gift = inventory1.gift(None);
    println!("{:#?}", my_gift);

    let data = |num: i32| -> i32 { num };
    data(23);
    borrow_closure();
    println!("--------------");
    borrow_mutably_closure();
    println!("--------------");
    manually_move_closure();
    ex_of_fn_once(|| println!("ex_of_fn_once"));
    let mut count = 0;
    let inc = || {
        count += 1;
        println!("Count = {count}");
    };
    ex_of_fn_mut(inc);
    let s = String::from("hello");

    call_fn_once(|| {
        println!("{}", s);
        drop(s);
    });
    iter_ex();
    iter_next_method();
    consume_iter();
    produce_other_iter();
}

fn iter_ex() {
    let num: Vec<i32> = vec![1, 2, 3];
    let num_iter = num.iter();
    for i in num_iter {
        println!("{:#?}", i);
    }
    dbg!(num);
}
fn iter_next_method() {
    let num: Vec<i32> = vec![1, 2, 3];
    let mut num_iter = num.iter();

    let i = num_iter.next();
    println!("{:#?}", i);
}

fn consume_iter() {
    let num = vec![1, 2, 3];

    let total: i32 = num.iter().sum();
    println!("{:#?}", total);
}

fn produce_other_iter() {
    let num = vec![1, 2, 3];
    let num_pluse_one:Vec<i32>=num.iter().map(|n|n+1).collect();
    dbg!(num_pluse_one);
}

fn ex_of_fn_once<F: Fn()>(f: F) {
    f();
    f();
}
fn ex_of_fn_mut<F>(mut f: F)
where
    F: FnMut(),
{
    f();
    f();
}
fn call_fn_once<F>(f: F)
where
    F: FnOnce(),
{
    f();
    // f();
}

fn borrow_closure() {
    let num: Vec<i32> = vec![12, 3, 4];
    println!("before define:{:#?}", num);

    let borrow = || println!("closer:{:#?}", num);
    println!("after define:{:#?}", num);
    borrow();
    println!("After called:{:#?}", num);
    borrow();
}
fn borrow_mutably_closure() {
    let mut num: Vec<i32> = vec![12, 3, 4];
    println!("before define:{:#?}", num);

    let mut borrow_fn = || num.push(20);
    // println!("after define:{:#?}", num); // not will work
    borrow_fn();
    println!("After called:{:#?}", num);
    // borrow_fn();
}

fn manually_move_closure() {
    let num: Vec<i32> = vec![12, 3, 4];
    println!("before define:{:#?}", num);
    thread::spawn(move || println!("inside thread: {:#?}", num));
    // println!("After called:{:#?}", num); //error
}
