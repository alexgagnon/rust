// enums are good for small enumerations (collections with a small amount of types)
#[derive(Debug)]
enum IPAddressEnum {
  V4,
  V6,
}

// Storing an enum in a struct
#[derive(Debug)]
struct IPAddress {
  kind: IPAddressEnum,
  address: String,
}

// enums are super useful because you can give them types and values, making them like small structs with a common trait
// you can put any type in an enum, even structs
#[derive(Debug)]
enum IPAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

fn main() {
  let localhost = IPAddress {
    kind: IPAddressEnum::V4,
    address: String::from("127.0.0.1"),
  };

  let loopback = IPAddr::V6(String::from("::1"));
  let private = IPAddr::V4(10, 148, 178, 1);

  println!("{:#?}", localhost);
  println!("{:#?}", loopback);
  println!("{:#?}", private);
}
