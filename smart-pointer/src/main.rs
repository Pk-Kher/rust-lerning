use std::ops::Deref;

fn main() {
    println!("Hello, world!");
    box_smart_pointer();
    regular_ref_with_deref();
    defining_own_smart_pointer();
    let s = String::new();
    // drop(s); //move
    // { s }; // move 
    // (|_| ())(s); //move
}

fn box_smart_pointer() {
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
fn regular_ref_with_deref() {
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
fn defining_own_smart_pointer() {
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(value: T) -> MyBox<T> {
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
