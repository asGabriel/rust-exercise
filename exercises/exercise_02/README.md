# Convert String to CamelCase

## Problem Statement
Write a function that converts a given **string** into **camelCase**.  
- All words must have their **first letter capitalized**.  
- **Spaces must be removed**.

## Examples
| Input            | Output        |
|-----------------|--------------|
| `"hello case"`  | `"HelloCase"` |
| `"camel case word"` | `"CamelCaseWord"` |

## Example Usage
```rust
fn main() {
    let result = to_camel_case("hello case");
    println!("{}", result); // Should print "HelloCase"
}
