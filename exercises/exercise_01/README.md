# Exercise: Convert to Roman Numerals

## Description
Create a function that takes a **positive integer** between **1 and 3999** (inclusive) and returns a string representing its **Roman numeral** representation.

Modern Roman numerals are written by expressing each digit separately from left to right, skipping any digit with a value of zero.

### Examples:
| Number | Roman Numeral |
|--------|--------------|
| 1990   | MCMXC        |
| 2008   | MMVIII       |
| 1666   | MDCLXVI      |
| 1000   | M            |

### Symbol Table:
| Symbol | Value |
|--------|-------|
| I      | 1     |
| V      | 5     |
| X      | 10    |
| L      | 50    |
| C      | 100   |
| D      | 500   |
| M      | 1000  |

**Note:** There cannot be more than **three identical symbols in a row**.

## Example Usage
```rust
fn main() {
    let result = solution(1000);
    println!("{}", result); // Should print "M"
}
