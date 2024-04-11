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

pub fn another_way_to_multiplication_table(len: usize) -> Vec<Vec<usize>> {
    (1..=len)
        .map(|i| (1..=len).map(|j| i * j).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(multiplication_table(3), [[1, 2, 3], [2, 4, 6], [3, 6, 9]]);
    }

    #[test]
    fn another_way() {
        assert_eq!(
            another_way_to_multiplication_table(3),
            [[1, 2, 3], [2, 4, 6], [3, 6, 9]]
        );
    }
}
