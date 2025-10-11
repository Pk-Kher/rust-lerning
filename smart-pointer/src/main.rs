use crate::ref_call_example::start_fn;
use std::{cell::RefCell, ops::Deref, rc::Rc};

pub mod ref_call_example;
// use crate::box_examples::dynamic_dispatch;
//
// pub mod box_examples;
fn main() {
    // box_smart_pointer();
    // regular_ref_with_deref();
    // defining_own_smart_pointer();
    // let s = String::new();
    // too_large_data();
    // dynamic_dispatch();
    // drop(s); //move
    // { s }; // move
    // (|_| ())(s); //move
    // rc_pointer_ex();
    start_fn();
    memory_leak_using_rc();
}
fn memory_leak_using_rc() {
    // #[derive(Debug)]
    // enum List {
    //     Cons(i32, RefCell<Rc<List>>),
    //     Nil,
    // }
    // impl List {
    //     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
    //         match self {
    //             List::Cons(_, item) => Some(item),
    //             List::Nil => None,
    //         }
    //     }
    // }
    // let a = Rc::new(List::Cons(23, RefCell::new(Rc::new(List::Nil))));
    // let b = Rc::new(List::Cons(21, RefCell::new(Rc::clone(&a))));
    // println!("b next item = {:?}", b.tail());
    //
    // if let Some(link) = a.tail() {
    //     *link.borrow_mut() = Rc::clone(&b);
    // }
    // println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // println!("a next item = {:?}", a.tail());

    // #[derive(Debug)]
    // struct Node {
    //     value: i32,
    //     parent: RefCell<Option<Rc<Node>>>,
    //     children: RefCell<Vec<Rc<Node>>>,
    // }
    // let parent = Rc::new(Node {
    //     value: 1,
    //     parent: RefCell::new(None),
    //     children: RefCell::new(vec![]),
    // });
    // let child = Rc::new(Node {
    //     value: 2,
    //     parent: RefCell::new(Some(Rc::clone(&parent))),
    //     children: RefCell::new(vec![]),
    // });
    // parent.children.borrow_mut().push(Rc::clone(&child));
    // println!(
    //     "Parent strong = {}, weak = ???",
    //     Rc::strong_count(&parent)
    // );
}
// fn rc_pointer_ex() {
//     enum List {
//         Cons(i32, Rc<List>),
//         Nail,
//     }
//
//     let a = Rc::new(List::Cons(
//         34,
//         Rc::new(List::Cons(34, Rc::new(List::Cons(56, Rc::new(List::Nail))))),
//     )); // increase counter to 1
//     println!("rc counter {}", Rc::strong_count(&a));
//     let b = List::Cons(234, Rc::clone(&a)); // increase counter to 2
//     println!("rc counter {}", Rc::strong_count(&a));
//     let c = List::Cons(3, Rc::clone(&a)); // increase counter to 3
//     println!("rc counter {}", Rc::strong_count(&a));
//     fn print_list(data: &List) {
//         match data {
//             List::Cons(value, next) => {
//                 println!("{}", value);
//                 print_list(next);
//             }
//             List::Nail => (),
//         }
//     }
//     print_list(&a);
//     struct Example {
//         data: i32,
//     };
//     impl Drop for Example {
//         fn drop(&mut self) {
//             println!("drop called !==");
//         }
//     }
//     let mut ex1 = Rc::new(Example { data: 34 });
//     let ex2 = Rc::clone(&ex1);
//     let mut normal_st = Example { data: 334 };
//     normal_st.data = 43;
//     // ex1.data=34;
//     println!("A");
//     drop(ex1);
//     println!("B");
//     drop(ex2);
//     println!("C");
// }

fn _box_smart_pointer() {
    let b = Box::new(3);
    println!("{}", b);

    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>), // if you put only list here then it will give you error
        Nil,
    }

    let data = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    dbg!(data);
    let mut n = 1;
    let b = Box::new(&mut n);
    **b += 1;
    println!("{}", b);
}
fn _regular_ref_with_deref() {
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
fn defining_own_smart_pointer() {
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(value: T) -> Self {
            MyBox(value)
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            print!("pk");
            &self.0
        }
    }
    impl<T> Drop for MyBox<T> {
        fn drop(&mut self) {
            println!("drop from mybox");
        }
    }

    let x = 4;
    let y = MyBox::new(4);
    // std::mem::drop(y); // manually drop
    assert_eq!(4, x);
    assert_eq!(4, *y);

    let m = MyBox(String::from("Rust"));
    // let str_slice = &m[..];
    hello(&m);
    hello(&(*m)[..]);
    fn hello(v: &str) {
        println!("hello {}", v);
    }
}

fn _extra() {
    #[derive(Clone, Copy)]
    struct AccessLogger(i32);

    impl Deref for AccessLogger {
        type Target = i32;

        fn deref(&self) -> &Self::Target {
            println!("deref");

            &self.0
        }
    }
    let n = AccessLogger(-1);

    let x = *n + 1;

    let n2 = n;

    println!("{} {}", x, *n)
}
