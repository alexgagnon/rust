fn main() {
    println!("Hello, world!");
    another_function();
    let x = 4;
    // this line shadows the previous x value
    let x = 10;
    println!("{}", x);
    {
        println!("{}", x);
        // this shadows the outer 'x', but only for this scope, provided by the {}
        let x = 20;
        println!("{}", x);
    }
    println!("{}", x);

    println!("{}", five());
}

fn another_function() -> i32 {
    println!("Another function!");
    return 0;
}

fn five() -> i32 {
    // note: no return or ';' here, as we want '5' to stay as an expression, not a statement
    5
}
