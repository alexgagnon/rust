#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = 30;
    let height = 50;
    let tuple = (30, 50);
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Using two variables: {}", area(width, height));

    println!("Using a tuple: {}", area_tuple(tuple));

    println!("Using a struct: {}", area_rectangle(&rectangle));

    println!("The rectangle: {:#?}", rectangle);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    let (width, height) = dimensions;
    width * height
}

fn area_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
