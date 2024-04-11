/*
Write a method (or function, depending on the language) that converts a string to camelCase, that is, all words must have their first letter capitalized and spaces must be removed.

Examples (input --> output):
"hello case" --> "HelloCase"
"camel case word" --> "CamelCaseWord"
*/

pub fn camel_case(str: &str) -> String {
    let split_str: Vec<&str> = str.trim().split(" ").collect();
    let mut result = String::new();

    for word in split_str {
        for (index, letter) in word.chars().enumerate() {
            match index {
                0 => result.push_str(&letter.to_uppercase().to_string().as_str()),
                _ => result.push_str(letter.to_lowercase().to_string().as_str()),
            }
        }
    }

    result
}

pub fn clever_camel_case(str: &str) -> String {
    str.split_whitespace()
        .map(|s| [&s[..1].to_uppercase(), &s[1..]].join(""))
        .collect()
}

#[cfg(test)]
mod test {
    use super::{camel_case, clever_camel_case};
    
    #[test]
    fn sample_test() {
        assert_eq!(camel_case("test case"), "TestCase");
        assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
        assert_eq!(camel_case("say hello "), "SayHello");
        assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
        assert_eq!(camel_case(""), "");
    }
    
    #[test]
    fn clever_test() {
        assert_eq!(clever_camel_case("test case"), "TestCase");
        assert_eq!(clever_camel_case("camel case method"), "CamelCaseMethod");
        assert_eq!(clever_camel_case("say hello "), "SayHello");
        assert_eq!(clever_camel_case(" camel case word"), "CamelCaseWord");
        assert_eq!(clever_camel_case(""), "");
    }
}
    