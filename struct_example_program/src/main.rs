#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, another: &Rectangle) -> bool {
        (self.width > another.width) && (self.height > another.height)
    }
   fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let my_rectangle = Rectangle {
        width: 30,
        height: 50
    };
    let my_second_rectangle = Rectangle::square(100);
    println!("The area of the rectangle is {}", my_rectangle.area());
    println!("The area of the rectangle is {}", area(&my_rectangle));
    println!("{}", my_rectangle.width);
    println!("{}", my_rectangle.width());
    println!("{}", my_rectangle.can_hold(&my_second_rectangle));
    dbg!(my_second_rectangle);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
