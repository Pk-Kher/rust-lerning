// use crate::rect::Rectangle;
//
// pub mod rect;
// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
fn print_and_return_10(value: i32) -> i32 {
    println!("print data is {value:}");
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_will_pass() {
        let value = print_and_return_10(4);
        assert_eq!(10, value);
    }
    #[test]
    fn test_will_fail() {
        let value = print_and_return_10(8);
        assert_eq!(value, 8);
    }

    // #[test]
    // fn exploration() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    //     assert_eq!(result, 5);
    // }
    // #[test]
    // fn another() {
    //     panic!("failed another");
    // }
    //
    // #[test]
    // // #[ignore]
    // fn larger_can_hold_smaller() {
    //     let rect1 = Rectangle::new(12, 20);
    //     let rect2 = Rectangle::new(10, 10);
    //     assert!(rect1.can_hold(&rect2));
    //     assert!(!rect2.can_hold(&rect1));
    // }
}
