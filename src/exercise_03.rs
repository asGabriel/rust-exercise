/*
MULTIPLICATION TABLE
Your task, is to create N×N multiplication table, of size provided in parameter.

For example, when given size is 3:
1 2 3
2 4 6
3 6 9
For the given example, the return value should be:

[[1,2,3],[2,4,6],[3,6,9]]
*/

pub fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
    let mut table: Vec<Vec<usize>> = Vec::with_capacity(len);

    for row in 1..=len {
        let mut vec_content: Vec<usize> = Vec::with_capacity(len);
        
        for content_counter in 1..=len {
            vec_content.push(content_counter * row);
        }

        table.push(vec_content)
    }

    table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(multiplication_table(3), [[1, 2, 3], [2, 4, 6], [3, 6, 9]]);
    }
}
