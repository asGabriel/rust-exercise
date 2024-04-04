/// Converts a number to a string representating roman numeral.
pub fn num_as_roman(num: i32) -> String {
    todo!()
}

#[test]
fn returns_expected() {
  assert_eq!(num_as_roman(182), "CLXXXII");
  assert_eq!(num_as_roman(1990), "MCMXC");
  assert_eq!(num_as_roman(1875), "MDCCCLXXV");
}
