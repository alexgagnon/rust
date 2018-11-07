pub mod a {
  pub mod series {
    pub mod of {
      pub fn nested_modules() {}
    }
  }
}

use self::a::series::of;

#[derive(Debug)]
enum TrafficLight {
  Red,
  Yellow,
  Green,
}

use self::TrafficLight::{Red, Yellow};
/**
use foo::{
  bar::{self, Foo},
  baz::{*, quux::Bar}, // glob bring all public items, quux::Bar is a child modules Bar
}
 */

fn main() {
  of::nested_modules();
  println!("{:#?}", Red);
}
