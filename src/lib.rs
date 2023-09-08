/// Represents the string "zero".
const ZERO: &'static str = "zero";

/// Represents the prefix for negative numbers.
const SIGNED: &'static str = "minus ";

/// Separator used to join various parts of the number's text representation.
const SEPARATOR: &'static str = " and ";

/// Representations for one-digit numbers.
const ONES: [&'static str; 10] = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

/// Representations for the numbers 11 to 19.
const TEENS: [&'static str; 10] = ["", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];

/// Representations for multiples of ten.
const TENS: [&'static str; 10] = ["", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];

/// Represents the word for hundred.
const HUNDRED: &'static str = "hundred";

/// Representations for number scales.
const SCALES: [&'static str; 7] = ["", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion"];

/// Converts a given integer `number` to its cardinal string representation.
///
/// # Examples
///
/// ```
/// use rust_numerals::number_to_cardinal;
/// assert_eq!(number_to_cardinal(12345), "twelve thousand three hundred and forty-five");
/// ```
///
/// # Arguments
///
/// * `number` - The integer value to be converted to text.
///
/// # Returns
///
/// Returns a `String` representing the cardinal version of the input number.
pub fn number_to_cardinal(number: i64) -> String {
    if number == 0 {
        return ZERO.to_string();
    }

    let mut words = String::new();
    let mut num = if number < 0 { -number } else { number } as u64; // Use absolute value

    // Decompose number into chunks of 3 digits and convert to words
    let mut scale_idx = 0;
    while num > 0 {
        if let Some(chunk) = get_chunk_words(num % 1_000, scale_idx) {
            words.insert_str(0, &chunk);
        }
        num /= 1_000;
        scale_idx += 1;
    }

    if number < 0 {
        words.insert_str(0, SIGNED);
    }
    words.trim().to_string()
}

/// Converts a chunk of the number to its cardinal string representation and appends its scale.
///
/// # Arguments
///
/// * `chunk` - A part of the number to be converted, typically 3 digits long.
/// * `scale_idx` - The index representing the scale (thousand, million, etc.) of the chunk.
///
/// # Returns
///
/// Returns an `Option<String>` where the `Some` variant contains the cardinal representation of the chunk and the `None` variant is returned for chunks that are zeros.
fn get_chunk_words(chunk: u64, scale_idx: usize) -> Option<String> {
    if chunk == 0 {
        return None;
    }
    
    let mut result = convert_number_to_cardinal(chunk as i64);
    if scale_idx > 0 {
        result.push_str(&format!(" {} ", SCALES[scale_idx]));
    }
    Some(result)
}

/// Converts a number (up to 999) to its cardinal string representation.
///
/// # Arguments
///
/// * `number` - The integer value (up to 999) to be converted to text.
///
/// # Returns
///
/// Returns a `String` representing the cardinal version of the input number.
fn convert_number_to_cardinal(number: i64) -> String {
    let mut words = String::new();

    // Handle hundreds place
    if number >= 100 {
        words.push_str(&format!("{} {}", ONES[(number / 100) as usize], HUNDRED));
        if number % 100 != 0 {
            words.push_str(SEPARATOR);
        }
    }

    let remainder = number % 100;

    // Handle tens and ones places
    match remainder {
        11..=19 => words.push_str(TEENS[(remainder - 10) as usize]),
        1..=9 => words.push_str(ONES[remainder as usize]),
        10 => words.push_str(TENS[1]),
        _ => {
            if remainder >= 20 {
                words.push_str(TENS[remainder as usize / 10]);
                if remainder % 10 != 0 {
                    words.push_str(&format!("-{}", ONES[(remainder % 10) as usize]));
                }
            }
        }
    }
    words
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_ones() {
        assert_eq!(number_to_cardinal(0), "zero");
        assert_eq!(number_to_cardinal(1), "one");
        assert_eq!(number_to_cardinal(2), "two");
        assert_eq!(number_to_cardinal(3), "three");
        assert_eq!(number_to_cardinal(4), "four");
        assert_eq!(number_to_cardinal(5), "five");
        assert_eq!(number_to_cardinal(6), "six");
        assert_eq!(number_to_cardinal(7), "seven");
        assert_eq!(number_to_cardinal(8), "eight");
        assert_eq!(number_to_cardinal(9), "nine");
    }

    #[test]
    fn it_works_with_signed_ones() {
        assert_eq!(number_to_cardinal(-1), "minus one");
        assert_eq!(number_to_cardinal(-2), "minus two");
        assert_eq!(number_to_cardinal(-3), "minus three");
        assert_eq!(number_to_cardinal(-4), "minus four");
        assert_eq!(number_to_cardinal(-5), "minus five");
        assert_eq!(number_to_cardinal(-6), "minus six");
        assert_eq!(number_to_cardinal(-7), "minus seven");
        assert_eq!(number_to_cardinal(-8), "minus eight");
        assert_eq!(number_to_cardinal(-9), "minus nine");
        assert_eq!(number_to_cardinal(-10), "minus ten");
    }

    #[test]
    fn it_works_with_teens() {
        assert_eq!(number_to_cardinal(11), "eleven");
        assert_eq!(number_to_cardinal(12), "twelve");
        assert_eq!(number_to_cardinal(13), "thirteen");
        assert_eq!(number_to_cardinal(14), "fourteen");
        assert_eq!(number_to_cardinal(15), "fifteen");
        assert_eq!(number_to_cardinal(16), "sixteen");
        assert_eq!(number_to_cardinal(17), "seventeen");
        assert_eq!(number_to_cardinal(18), "eighteen");
        assert_eq!(number_to_cardinal(19), "nineteen");
    }

    #[test]
    fn it_works_with_tens() {
        assert_eq!(number_to_cardinal(10), "ten");
        assert_eq!(number_to_cardinal(20), "twenty");
        assert_eq!(number_to_cardinal(21), "twenty-one");
        assert_eq!(number_to_cardinal(30), "thirty");
        assert_eq!(number_to_cardinal(33), "thirty-three");
        assert_eq!(number_to_cardinal(40), "forty");
        assert_eq!(number_to_cardinal(50), "fifty");
        assert_eq!(number_to_cardinal(60), "sixty");
        assert_eq!(number_to_cardinal(70), "seventy");
        assert_eq!(number_to_cardinal(80), "eighty");
        assert_eq!(number_to_cardinal(90), "ninety");
    }

    #[test]
    fn it_works_with_thousands() {
        assert_eq!(number_to_cardinal(1_000), "one thousand");
        assert_eq!(number_to_cardinal(1_001), "one thousand one");
        assert_eq!(number_to_cardinal(1_100), "one thousand one hundred");
        assert_eq!(number_to_cardinal(10_000), "ten thousand");
        assert_eq!(number_to_cardinal(11_000), "eleven thousand");
        assert_eq!(number_to_cardinal(11_011), "eleven thousand eleven");
        assert_eq!(number_to_cardinal(21_025), "twenty-one thousand twenty-five");
        assert_eq!(number_to_cardinal(99_999), "ninety-nine thousand nine hundred and ninety-nine");
    }

    #[test]
    fn it_works_with_larger_scales() {
        assert_eq!(number_to_cardinal(1_000_000), "one million");
        assert_eq!(number_to_cardinal(1_000_001), "one million one");
        assert_eq!(number_to_cardinal(1_001_000), "one million one thousand");
        assert_eq!(number_to_cardinal(1_001_001), "one million one thousand one");
        assert_eq!(number_to_cardinal(1_061_044), "one million sixty-one thousand forty-four");

        assert_eq!(number_to_cardinal(1_000_000_000), "one billion");
        assert_eq!(number_to_cardinal(1_000_000_001), "one billion one");
        assert_eq!(number_to_cardinal(1_000_001_000), "one billion one thousand");
        assert_eq!(number_to_cardinal(1_000_001_001), "one billion one thousand one");
        assert_eq!(number_to_cardinal(1_000_456_501), "one billion four hundred and fifty-six thousand five hundred and one");

        assert_eq!(number_to_cardinal(1_000_000_000_000), "one trillion");
        assert_eq!(number_to_cardinal(1_000_000_000_000_000), "one quadrillion");
        assert_eq!(number_to_cardinal(1_000_000_000_000_000_000), "one quintillion");

        assert_eq!(number_to_cardinal(2_000_000_000_000), "two trillion");
        assert_eq!(number_to_cardinal(2_000_000_000_000_000), "two quadrillion");
        assert_eq!(number_to_cardinal(2_000_000_000_000_000_000), "two quintillion");
        assert_eq!(number_to_cardinal(2_000_892_000_560_056_000), "two quintillion eight hundred and ninety-two trillion five hundred and sixty million fifty-six thousand");

        assert_eq!(number_to_cardinal(1_001_000_000_000), "one trillion one billion");
        assert_eq!(number_to_cardinal(1_000_001_000_000_000), "one quadrillion one billion");
        assert_eq!(number_to_cardinal(1_000_000_001_000_000_000), "one quintillion one billion");
        assert_eq!(number_to_cardinal(1_000_892_000_560_056_000), "one quintillion eight hundred and ninety-two trillion five hundred and sixty million fifty-six thousand");

        assert_eq!(number_to_cardinal(1_001_000_000_001), "one trillion one billion one");
        assert_eq!(number_to_cardinal(1_000_001_000_000_001), "one quadrillion one billion one");
        assert_eq!(number_to_cardinal(1_000_000_001_000_000_001), "one quintillion one billion one");

        assert_eq!(number_to_cardinal(1_100_100_100_100_100_101), "one quintillion one hundred quadrillion one hundred trillion one hundred billion one hundred million one hundred thousand one hundred and one");
    }
}
