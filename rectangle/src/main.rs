#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn calculate_area(self: &Self) -> u32 {
        self.width * self.height
    }

    fn can_hold(self: &Self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let scale: u32 = 1;
    let rectangle_1: Rectangle = Rectangle {
        width: dbg!(scale * 50),
        height: 30,
    };
    let rectangle_2: Rectangle = Rectangle {
        width: 49,
        height: 29,
    };
    let rectangle_3: Rectangle = Rectangle {
        width: 51,
        height: 31,
    };

    println!("rectangle_1 is {:#?}", rectangle_1);
    println!("The area is {}", rectangle_1.calculate_area());
    println!(
        "Can it hold rectangle_2? {}",
        rectangle_1.can_hold(&rectangle_2)
    );
    println!(
        "Can it hold rectangle_3? {}",
        rectangle_1.can_hold(&rectangle_3)
    );
}
