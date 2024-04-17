/*
Break camelCase
Complete the solution so that the function will break up camel casing, using a space between words.

Example
"camelCasing"  =>  "camel Casing"
"identifier"   =>  "identifier"
""             =>  ""
*/

pub fn break_camel_case(s: &str) -> String {
    s.chars().into_iter().fold(String::new(), |mut word, char| {
        if char.is_ascii_uppercase() {
            word.push_str(" ")
        }
        word.push(char);
        word
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(break_camel_case("camelCasing"), "camel Casing");
        assert_eq!(break_camel_case("camelCasingTest"), "camel Casing Test");
    }
}
