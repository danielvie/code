#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
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
    
    fn magic() -> Self {
        Self {
            width: 9,
            height: 9,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };

    println!("the area of the rectangle is {} area units", rect1.area());

    let rect2 = Rectangle::square(10);   
    
    let rect3 = Rectangle {
        width: 40,
        height: 10,
    };
    
    let magic = Rectangle::magic();
    
    println!("rect1 ({:?}) can hold rect2 ({:?}): {}", rect1, rect2, rect1.can_hold(&rect2));
    println!("rect1 ({:?}) can hold rect3 ({:?}): {}", rect1, rect3, rect1.can_hold(&rect3));
    println!("magic ({:?})", magic);
}
