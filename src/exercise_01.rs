/*
Create a function taking a positive integer between 1 and 3999 (both included) as its parameter and returning a string containing the Roman Numeral representation of that integer.

Modern Roman numerals are written by expressing each digit separately starting with the left most digit and skipping any digit with a value of zero. In Roman numerals 1990 is rendered: 1000=M, 900=CM, 90=XC; resulting in MCMXC. 2008 is written as 2000=MM, 8=VIII; or MMVIII. 1666 uses each Roman symbol in descending order: MDCLXVI.

Example:

solution(1000); // should return 'M'
Help:

Symbol    Value
I          1
V          5
X          10
L          50
C          100
D          500
M          1,000

Remember that there can't be more than 3 identical symbols in a row.
*/

/// Converts a number to a string representating roman numeral.
pub fn num_as_roman(mut num: i32) -> String {
    let arabic_values = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let roman_values = vec!["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
    let mut result = String::new();

    for (i, &arabic) in arabic_values.iter().enumerate() {
        while num >= arabic {
            num -= arabic;
            result.push_str(roman_values[i]);
        }
    }
    println!("EX 01: The Arabic number {} in Roman is {}", num, result);
    result
}

#[test]
fn returns_expected() {
  assert_eq!(num_as_roman(182), "CLXXXII");
  assert_eq!(num_as_roman(1990), "MCMXC");
  assert_eq!(num_as_roman(1875), "MDCCCLXXV");
}
