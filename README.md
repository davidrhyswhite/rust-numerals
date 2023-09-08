# Rust Numerals

Crate for converting `i64` integers into their cardinal string format.

### TODO

- [ ] Support localisation;
- [ ] Comma-support inside large strings;
- [ ] `number_to_ordinal` method for ordinals;
- [ ] `number_to_roman` method for Roman ordinals;
- [ ] Support multiple integer types `i8 .. i32` & `u8 .. u64`;
- [ ] Support multiple integer types `i8 .. i32` & `u8 .. u64`;
- [ ] Support floating point numbers `f32` & `f64`;

## Getting started

Add the following lines to your `Cargo.toml` dependencies:

```toml
[dependencies]
rust-numerals = "0.1.0"
```

## Examples

Convert a number to it's ordinal words.

```rust
use rust_numerals::number_to_cardinal;

fn main() {
    let number: i64 = 23;
    let cardinal = number_to_cardinal(23);
    println!("{} == {}", number, cardinal); // 23 == twenty-three
}
```

### More examples

Using the `i64` we can get numbers into the quintillions, which is probably larger than most projects needs.

```rust
number_to_cardinal(0); // zero
number_to_cardinal(8); // eight
number_to_cardinal(14); // fourteen
number_to_cardinal(23); // twenty-three
number_to_cardinal(108); // one hundred eight
number_to_cardinal(1256); // one thousand two hundred and fifty-six
number_to_cardinal(1_100); // one thousand one hundred
number_to_cardinal(11_011); // eleven thousand eleven
number_to_cardinal(21_025); // twenty-one thousand twenty-five
number_to_cardinal(99_999); // ninety-nine thousand nine hundred and ninety-nine
number_to_cardinal(1_001_000); // one million one thousand
number_to_cardinal(1_061_044); // one million sixty-one thousand forty-four
number_to_cardinal(2_000_000_000_000); // two trillion
number_to_cardinal(2_000_892_000_560_056_000); // two quintillion eight hundred and ninety-two trillion five hundred and sixty million fifty-six thousand
number_to_cardinal(1_100_100_100_100_100_101); // one quintillion one hundred quadrillion one hundred trillion one hundred billion one hundred million one hundred thousand one hundred and one
```

## License

Licensed under MIT license ([LICENSE](LICENSE) or https://opensource.org/licenses/MIT).