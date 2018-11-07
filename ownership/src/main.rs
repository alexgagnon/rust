// each value in Rust has an 'owner'
// there can only be one owner at a time
// when the owner goes out of scope, the value is dropped
// references are 'moved', primitives are copied
// 'primitives' are types with the Copy trait

// All the integer types, such as u32.
// The Boolean type, bool, with values true and false.
// All the floating point types, such as f64.
// The character type, char.
// Tuples, but only if they contain types that are also Copy. For example, (i32, i32) is Copy, but (i32, String) is not.

// REMEMBER: for heap memory, the backing memory will be dropped when the owner goes out of scope, important when you pass in an object into a function as you'd need to return it back to keep it around!

fn main() {
    let _s = "Hello"; // a string literal, stored as is in the text segment
    let mut s = String::from("Hello"); // string object, stored in the heap. We need this because to resize it, we need to be able to allocate more memory from the heap
    s.push_str(", world!");

    let t = s; // this is a 'move', it invalidates using 's' as a variable as 't' is now the owner, and the memory can be freed when 't' goes out of scope
               // println!("{}", s); // fails!
    println!("{}", t);

    let x = 5;
    let y = x; // this is fine, as variables that are primitives (they have the Copy trait) or literals are copied (they have a known size at compile time and exist on the stack (primitives) or in the text segment (literals)), unlike pointer references to places in the heap
    let m = "thing";
    let n = m;
    println!("{} {} {} {}", x, y, m, n);

    takes_ownership(t); // this moves the reference into the some_string variable of the function!!!!
                        // println!("{}", t); // no longer valid, as it was dropped after the function went out of scope!

    makes_copy(x);
    println!("{}", x); // fine, primitives are copied!

    let s3 = gives_ownership();
    println!("{}", s3);

    // if you want an heap object to exist after passing it as an argument to a function, reference it with & instead
    let goodbye = String::from("Goodbye");
    borrows(&goodbye);
    println!("{} is still here", goodbye);
    // the function 'borrows' the value, it doesn't take ownership so it isn't allowed to modify it and must return it when it's done

    // to have the borrowed value be mutable, need to use types 'mut' and '&mut'
    let mut changeable = String::from("change me!");
    borrows_mut(&mut changeable);
    println!("{}", changeable);

    // NOTE you can only have one reference to a mutable value at a time
    let c1 = &mut changeable;
    // let c2 = &mut changeable; // fails, already have a reference in c1
    println!("{}", c1);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    return some_string; // transfers ownership out of the function!
}

fn borrows(string: &String) {
    // string.push_str("uh oh!"); // illegal, can't modify non-mut borrowed values
    println!("{}", string);
}

fn borrows_mut(string: &mut String) {
    println!("{}", string);
    string.push_str(" yeah!"); // fine
}
