use core::slice;
use std::ops::Add;

#[allow(dead_code, unused, unused_mut)]
fn main() {
    // dereferenceing_raw_pointer();
    // unsafe_functions();
    // calling_other_lang_fn();
    // static_var();
    // advance_traits();
    // never_type();
    macros_ex();
}

fn dereferenceing_raw_pointer() {
    let mut x = 10;
    let r1 = &x; // immutable reference
    // let r2 = &mut x;  //  ERROR: cannot borrow `x` as mutable because it's already borrowed immutably
    println!("{}", r1);

    let mut num = 5;
    let r3: *const i32 = &raw const num; // raw immutable pointer
    let r4: *mut i32 = &raw mut num; // raw mutable pointer
    unsafe {
        *r4 = 10;
        println!("{:?}", *r3);
    }
}

fn unsafe_functions() {
    unsafe fn dangerous() {
        // unsafe {}
    }
    unsafe {
        dangerous();
    }
    let mut data = vec![1, 2, 3, 4, 5, 6];
    println!("{:#?}", split_at_mut(&mut data, 2));
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let length = values.len();
        let ptr = values.as_mut_ptr();
        assert!(mid <= length);
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), length - mid),
            )
        }
    }
}

fn calling_other_lang_fn() {
    unsafe extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("c code{}", abs(-3));
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
}

fn static_var() {
    static HELLO_WORLD: &str = "Hello, world!";
    println!("name is: {HELLO_WORLD}");
    static mut COUNTER: u32 = 0;
    unsafe fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}

fn advance_traits() {
    struct Counter {}
    impl Iterator for Counter {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            Some(2)
        }
    }

    let _count = Counter {};

    // for i in count {
    //     println!("{}", i);
    // }
    operator_overloading();
    fn operator_overloading() {
        #[derive(Debug, Clone, Copy)]
        struct Point {
            x: u32,
            y: u32,
        }

        impl Add for Point {
            type Output = Point;

            fn add(self, rhs: Self) -> Self::Output {
                Point {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        let point1 = Point { x: 1, y: 2 };
        let point2 = Point { x: 1, y: 2 };
        let sum = point1 + point2;
        println!("{:#?}", sum);
    }
}

fn never_type() -> ! {
    panic!("never return");
}
#[macro_export]
macro_rules! my_vec {
        ($($x:expr),*) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
fn macros_ex() {
    //Declarative Macros with macro_rules!
    let data = my_vec!["pk","data"];
    println!("{:#?}",data);
}
