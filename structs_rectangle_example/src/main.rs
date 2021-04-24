#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        (self.width + self.height) * 2
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // This is an associated function because it doesn't take in a `self`
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
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

    let square = Rectangle::square(29);

    println!(
        "rect1 area is {}, perimeter is {}, and rect1 can hold rect2 {}, and rect2 can hold square {}",
        rect1.area(),
        rect1.perimeter(),
        rect1.can_hold(&rect2),
        rect2.can_hold(&square)
    );
}
