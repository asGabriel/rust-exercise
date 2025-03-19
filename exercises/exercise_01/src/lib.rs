/// Converts a number to a string representating roman numeral.
pub fn num_as_roman(mut num: i32) -> String {
    let arabic_values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let roman_values = vec![
        "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
    ];
    let mut result = String::new();

    for (i, &arabic) in arabic_values.iter().enumerate() {
        while num >= arabic {
            num -= arabic;
            result.push_str(roman_values[i]);
        }
    }
    result
}

#[test]
fn returns_expected() {
    assert_eq!(num_as_roman(182), "CLXXXII");
    assert_eq!(num_as_roman(1990), "MCMXC");
    assert_eq!(num_as_roman(1875), "MDCCCLXXV");
}
