// REMEMBER: everything is immutable by default, use mut is it's okay to change it

// vectors only allow a single type

fn main() {
  // vectors are indexable collections that can grow/shrink
  let mut v1: Vec<i32> = Vec::new(); // here we need the give type since we use new
  let mut v2 = vec![1, 2, 3]; // here type is inferred when using the vec! macro

  v1.push(5);
  v2.push(6);

  {
    // scoping rules are the same
    let scoped_v3 = vec![1, 2, 3];
  }
  // here scoped_v3 go out
  // IMPORTANT: if the items are references, not only is the vector dropped, but ALL the elements within it are also

  // to access values, either dereference and use indexing
  println!("{}", &v1[0]);

  // or use the 'get' method
  println!("{}", v1.get(0).unwrap());

  // let a1 = &v1[100]; // causes a panic due to index out of bounds
  let a2 = v1.get(100); // would just return None from Option<T>

  // you cant have immutable references to any elements and then try to mutate the collection

  // immutable reference for immutable vector
  for i in &vec![1, 2, 3] {
    println!("{}", i);
  }

  // mutable vector
  for i in &mut vec![7, 8, 9] {
    *i += 50; // can mutate
    println!("{}", i);
  }

  // since enums have the same traits, you can store all of them in a vector, even if they have different types
  enum Thing {
    Int(i32),
    Str(String),
  }

  let animals = vec![Thing::Int(3), Thing::Str(String::from("hello"))];

  // String is owned, mutable type provided by the standard lib, while str is part of the core language and is immutable by default
  let mut s = String::from("foo");
  s.push_str("other");
  println!("{}", s);
  let s1 = String::from("Hello, ");
  let s2 = String::from("World!");
  s = s1 + &s2;
  println!("{} {}", s, s2);

  // will fail since s1 was used in the +'s 'add' operator
  // println!("{}", s1);

  // because strings in Rust are unicode, you can't index directly
  // need to convert them into characters from the underlying byte array first
  for c in "Thing".chars() {
    println!("{}", c);
  }
}
