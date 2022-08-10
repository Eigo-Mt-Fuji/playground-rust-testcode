// extern crate adder;

mod tests {
  use adder::add;

  #[test]
  fn test_add() {
    let res = add(1,2);
    assert_eq!(res, 3);
  }
}
