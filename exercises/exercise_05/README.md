# Is a Number Prime?

## Problem Statement
Write a function that takes an **integer** as input and returns a **boolean value** (`true` or `false`) indicating whether the number is **prime**.

### Definition
A **prime number** is a natural number **greater than 1** that has **no positive divisors** other than `1` and itself.

### Requirements
- The input will always be an integer.  
- The number **can be negative or zero**, but only positive numbers greater than `1` can be prime.  
- **Performance Consideration:** Numbers can be as large as `2^31`. Avoid trivial solutions that iterate up to `n` or `n/2`, as they may be too slow.

## Example
```rust
assert_eq!(is_prime(1), false);
assert_eq!(is_prime(2), true);
assert_eq!(is_prime(-1), false);
