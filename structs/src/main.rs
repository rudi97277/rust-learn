struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 20,
    };

    let rect2 = Rectangle {
        width: 31,
        height: 22,
    };

    let rect3 = Rectangle::square(20);

    println!("value {}", rect3.area());

    println!("value {}", rect1.area());
    println!("if first can hold the second {}", rect2.can_hold(&rect1));
}
