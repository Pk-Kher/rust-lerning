#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}
impl Rectangle {

    //associations
    fn square(size: i32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    //methods
    fn area(&self) -> i32 {
        self.height * self.width
    }
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }
}
fn main() {
    // let area_value=area(10,10);
    // let rect: (i32, i32) = (10, 10);
    // let area_value = area(rect);
    let rect1 = Rectangle {
        height: 10,
        width: 40,
    };
    let rect2 = Rectangle {
        height: 9,
        width: 30,
    };
    let rect3 = Rectangle {
        height: 20,
        width: 40,
    };
    let rect4 = Rectangle::square(32);
    // let area_value = area(&mut data);
    // let area_value=area(&data);
    // dbg!(&data);
    println!(
        "{:#?} {:#?} {:#?} {:#?}",
        rect1.area(),
        rect1.can_hold(&rect2),
        rect1.can_hold(&rect3),
        rect4
    );
}
// fn area(data: &Rectangle) -> i32 {
//     data.height * data.width
// }
// fn area(dimensions: (i32, i32)) -> i32 {
//     dimensions.0 * dimensions.1
// }
// fn area(width: i32, height: i32) -> i32 {
//     width * height
// }
