use rust_numerals::number_to_cardinal;

fn example(number: i64) {
    println!("{} == {}", number, number_to_cardinal(number));
}

fn main() {
    example(0);
    example(3);
    example(10);
    example(17);
    example(23);
    example(105);
}