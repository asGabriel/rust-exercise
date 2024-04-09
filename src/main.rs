mod exercise_01;
mod exercise_02;
fn main() {
    println!("Hello to exercises!");
    let ex01 = exercise_01::num_as_roman(8);
    println!("{}", ex01);

    let ex02 = exercise_02::camel_case("Lets go my brother");
    println!("{}", ex02);
    let ex02_clever = exercise_02::clever_camel_case("This is better than the another one");
    println!("{}", ex02_clever);
}
