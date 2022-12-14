#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    println!("The rectangle is {:#?}", rect1);

    println!("The rectangle has width: {}", rect1.width());

    println!("The area of the rectangle is {}.", rect1.area());

    println!("This is another rectangle {:#?}", rect2);

    println!(
        "Can the first rectangle hold the second? {}",
        rect1.can_hold(&rect2)
    );
}
