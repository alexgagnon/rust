// CONTINUED FROM STRUCT2 PACKAGE

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// use the 'impl` to define implementation details, such as methods
impl Rectangle {
    // NOTE: a method MUST have 'self' as the first argument, typically a borrow
    // can just do &self instead of self: Rectangle because the type is infered
    // since it's in an 'impl' block
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // associated functions are defined in the 'impl' block,
    // but don't take 'self' as the first parameter, so aren't
    // instance methods
    // they are called with ::, i.e. Rectangle::square
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// can have multiple 'impl' block
impl Rectangle {
    fn can_hold(&self, rect2: &Rectangle) -> bool {
        self.width > rect2.width && self.height > rect2.height
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 29,
        height: 49,
    };

    let rect3 = Rectangle {
        width: 31,
        height: 49,
    };

    // use method syntax (instance.method())
    println!("Using a struct method: {}", rectangle.area());

    println!("The rectangle: {:#?}", rectangle);

    println!("Rect2 can fit: {}", rectangle.can_hold(&rect2));
    println!("Rect3 can fit: {}", rectangle.can_hold(&rect3));

    println!("A square! {:#?}", Rectangle::square(20));
}
