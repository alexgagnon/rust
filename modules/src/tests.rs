use super::client;

#[test]
fn it_works() {
  assert_eq!(2 + 2, 4);
}

#[test]
fn do_it() {
  client::connect();
}
