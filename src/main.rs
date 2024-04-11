mod exercise_01;
mod exercise_02;
mod exercise_03;
fn main() {
    println!("Hello to exercises!");
    // EXERCISE 01
    let ex01 = exercise_01::num_as_roman(8);
    println!("{}", ex01);

    // EXERCISE 02
    let ex02 = exercise_02::camel_case("Lets go my brother");
    println!("{}", ex02);
    let ex02_clever = exercise_02::clever_camel_case("This is better than the another one");
    println!("{}", ex02_clever);

    // EXERCISE 03
    let ex03: Vec<Vec<usize>> = exercise_03::multiplication_table(2);
    println!("{:?}", ex03);
}
