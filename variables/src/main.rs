fn main() {
    let mut x = add(3, 4);
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum number of points is: {}", MAX_POINTS);

    let y = 10;
    println!("y: {}", y);
    let y = 20;
    println!("y: {}", y);
    let y = y + 5;
    println!("y: {}", y);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
