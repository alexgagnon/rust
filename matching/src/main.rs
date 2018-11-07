enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}

// match blocks are exhaustive, you must account for all possible values
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

// use '_' to reference 'everything else'
fn default_match(x: String) {
  match x {
    _ => (),
  }
}

fn main() {
  println!("{}", value_in_cents(Coin::Penny));

  let value = 5;

  println!("{} + 1 is: {}", value, plus_one(Some(value)).unwrap());

  default_match(String::from(""));

  let u8_value = Some(3u8);

  // if you only care about matching a specifiy value and ignoring everything else, this is long
  match u8_value {
    Some(3) => println!("three"),
    _ => (),
  }

  // use 'if let' construct instead, and you can include an else for everything else
  if let Some(3) = u8_value {
    println!("three");
  } else {

  }
}
