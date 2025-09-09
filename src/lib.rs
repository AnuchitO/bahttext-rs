use std::error::Error;
use std::fmt;

const UNIT_WORDS: [&str; 10] = [
    "",
    "หนึ่ง",
    "สอง",
    "สาม",
    "สี่",
    "ห้า",
    "หก",
    "เจ็ด",
    "แปด",
    "เก้า",
];

const UNIT_PLACES: [&str; 7] = ["", "สิบ", "ร้อย", "พัน", "หมื่น", "แสน", "ล้าน"];

pub fn words(money: f64) -> String {
    let minus = if money.is_sign_negative() {
        "ลบ"
    } else {
        ""
    };

    let precise_amount = (money.abs() * 100.0).round() / 100.0;
    let whole_baht = precise_amount.trunc() as u64;
    let satang = ((precise_amount - whole_baht as f64) * 100.0).round() as u64;

    let baht_text = money_to_thai_words(whole_baht);

    match satang {
        0 => format!("{}{}บาทถ้วน", minus, baht_text),
        _ => format!("{}{}บาท{}สตางค์", minus, baht_text, money_to_thai_words(satang)),
    }
}

fn money_to_thai_words(money: u64) -> String {
    if money == 0 {
        return "ศูนย์".to_string();
    }

    let mut baht_text = String::with_capacity(128);
    let mut amount = money;

    if amount >= 1_000_000 {
        let millions = amount / 1_000_000;
        amount %= 1_000_000;
        baht_text.push_str(&money_to_thai_words(millions));
        baht_text.push_str(UNIT_PLACES[6]);
    }

    let s = amount.to_string();
    let len_s = s.len();

    for (i, char) in s.chars().enumerate() {
        let digit = char.to_digit(10).unwrap() as usize;
        let place = len_s - i - 1;

        if digit == 0 {
            continue;
        }

        match (digit, place) {
            (2, 1) => baht_text.push_str("ยี่สิบ"),
            (1, 1) => baht_text.push_str(UNIT_PLACES[1]),
            (1, 0) if !baht_text.is_empty() => baht_text.push_str("เอ็ด"),
            _ => {
                baht_text.push_str(UNIT_WORDS[digit]);
                if place > 0 && place < UNIT_PLACES.len() {
                    baht_text.push_str(UNIT_PLACES[place]);
                }
            }
        }
    }

    baht_text
}

/// Error types for bahttext operations
#[derive(Debug, PartialEq)]
pub enum BahtTextError {
    /// Error when parsing string to number fails
    ParseError(String),
    /// Error when the amount is not a number
    InvalidNumber,
    /// Error when the amount is too large
    AmountTooLarge,
}

impl Error for BahtTextError {}

impl fmt::Display for BahtTextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "Failed to parse amount: {}", msg),
            Self::InvalidNumber => write!(f, "Invalid number format"),
            Self::AmountTooLarge => write!(f, "Amount is too large"),
        }
    }
}

/// Converts a string representing a monetary amount to Thai text representation.
///
/// # Arguments
/// * `input` - A string slice that holds the monetary amount (e.g., "1,234.56")
///
/// # Errors
/// Returns `BahtTextError` if the input cannot be parsed as a valid number
///
/// # Examples
/// ```
/// use bahttext::words_from;
///
/// let result = words_from("1,234.56").unwrap();
/// assert_eq!(result, "หนึ่งพันสองร้อยสามสิบสี่บาทห้าสิบหกสตางค์");
/// ```
pub fn words_from(input: &str) -> Result<String, BahtTextError> {
    let cleaned = input.trim().replace(',', "");
    cleaned.parse::<f64>()
        .map_err(|e| BahtTextError::ParseError(e.to_string()))
        .and_then(|n| {
            if n.is_finite() {
                Ok(words(n))
            } else {
                Err(BahtTextError::InvalidNumber)
            }
        })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_words_from() {
        // Test with integers
        assert_eq!(words_from("0").unwrap(), "ศูนย์บาทถ้วน");
        assert_eq!(words_from("100").unwrap(), "หนึ่งร้อยบาทถ้วน");

        // Test with decimal numbers
        assert_eq!(words_from("1.50").unwrap(), "หนึ่งบาทห้าสิบสตางค์");
        assert_eq!(words_from("100.25").unwrap(), "หนึ่งร้อยบาทยี่สิบห้าสตางค์");

        // Test with thousand separators
        assert_eq!(words_from("1,000").unwrap(), "หนึ่งพันบาทถ้วน");
        assert_eq!(words_from("1,234.56").unwrap(), "หนึ่งพันสองร้อยสามสิบสี่บาทห้าสิบหกสตางค์");

        // Test with whitespace
        assert_eq!(words_from(" 1234.56 ").unwrap(), "หนึ่งพันสองร้อยสามสิบสี่บาทห้าสิบหกสตางค์");
    }

    #[test]
    fn test_baht_text_error_display() {
        // Test ParseError
        let parse_error = BahtTextError::ParseError("invalid format".to_string());
        assert_eq!(
            parse_error.to_string(),
            "Failed to parse amount: invalid format",
            "Should format ParseError correctly"
        );

        // Test InvalidNumber
        let invalid_number = BahtTextError::InvalidNumber;
        assert_eq!(
            invalid_number.to_string(),
            "Invalid number format",
            "Should format InvalidNumber correctly"
        );

        // Test AmountTooLarge
        let amount_too_large = BahtTextError::AmountTooLarge;
        assert_eq!(
            amount_too_large.to_string(),
            "Amount is too large",
            "Should format AmountTooLarge correctly"
        );
    }

    #[test]
    fn test_words_from_error_cases() {
        // Test empty string and whitespace only
        assert!(matches!(
            words_from("").unwrap_err(),
            BahtTextError::ParseError(_)
        ), "Should return ParseError for empty string");

        assert!(matches!(
            words_from("   ").unwrap_err(),
            BahtTextError::ParseError(_)
        ), "Should return ParseError for whitespace only");

        // Test special characters
        assert!(matches!(
            words_from("@#$").unwrap_err(),
            BahtTextError::ParseError(_)
        ), "Should return ParseError for special characters");

        // Test invalid number formats
        assert!(matches!(
            words_from("abc").unwrap_err(),
            BahtTextError::ParseError(_)
        ), "Should return ParseError for non-numeric input");

        // Test infinity and NaN
        assert_eq!(
            words_from("inf").unwrap_err(),
            BahtTextError::InvalidNumber,
            "Should return InvalidNumber for infinity"
        );

        assert_eq!(
            words_from("NaN").unwrap_err(),
            BahtTextError::InvalidNumber,
            "Should return InvalidNumber for NaN"
        );
    }

    #[test]
    fn baht_to_words() {
        let cases = [
            // Zero and Single Digits
            ("zero", 0.0, "ศูนย์บาทถ้วน"),
            ("one", 1.0, "หนึ่งบาทถ้วน"),
            ("two", 2.0, "สองบาทถ้วน"),
            ("three", 3.0, "สามบาทถ้วน"),
            ("four", 4.0, "สี่บาทถ้วน"),
            ("five", 5.0, "ห้าบาทถ้วน"),
            ("six", 6.0, "หกบาทถ้วน"),
            ("seven", 7.0, "เจ็ดบาทถ้วน"),
            ("eight", 8.0, "แปดบาทถ้วน"),
            ("nine", 9.0, "เก้าบาทถ้วน"),
            // Tens
            ("ten", 10.0, "สิบบาทถ้วน"),
            ("twenty", 20.0, "ยี่สิบบาทถ้วน"),
            ("twenty-one", 21.0, "ยี่สิบเอ็ดบาทถ้วน"),
            ("twenty-two", 22.0, "ยี่สิบสองบาทถ้วน"),
            ("twenty-three", 23.0, "ยี่สิบสามบาทถ้วน"),
            ("twenty-four", 24.0, "ยี่สิบสี่บาทถ้วน"),
            ("twenty-five", 25.0, "ยี่สิบห้าบาทถ้วน"),
            ("twenty-six", 26.0, "ยี่สิบหกบาทถ้วน"),
            ("twenty-seven", 27.0, "ยี่สิบเจ็ดบาทถ้วน"),
            ("twenty-eight", 28.0, "ยี่สิบแปดบาทถ้วน"),
            ("twenty-nine", 29.0, "ยี่สิบเก้าบาทถ้วน"),
            ("thirty", 30.0, "สามสิบบาทถ้วน"),
            ("thirty-one", 31.0, "สามสิบเอ็ดบาทถ้วน"),
            ("forty", 40.0, "สี่สิบบาทถ้วน"),
            ("forty-one", 41.0, "สี่สิบเอ็ดบาทถ้วน"),
            ("fifty", 50.0, "ห้าสิบบาทถ้วน"),
            ("fifty-one", 51.0, "ห้าสิบเอ็ดบาทถ้วน"),
            ("fifty-five", 55.0, "ห้าสิบห้าบาทถ้วน"),
            ("sixty", 60.0, "หกสิบบาทถ้วน"),
            ("sixty-one", 61.0, "หกสิบเอ็ดบาทถ้วน"),
            ("seventy", 70.0, "เจ็ดสิบบาทถ้วน"),
            ("seventy-one", 71.0, "เจ็ดสิบเอ็ดบาทถ้วน"),
            ("eighty", 80.0, "แปดสิบบาทถ้วน"),
            ("eighty-one", 81.0, "แปดสิบเอ็ดบาทถ้วน"),
            ("ninety", 90.0, "เก้าสิบบาทถ้วน"),
            ("ninety-one", 91.0, "เก้าสิบเอ็ดบาทถ้วน"),
            ("ninety-nine", 99.0, "เก้าสิบเก้าบาทถ้วน"),
            // Hundreds
            ("one-hundred", 100.0, "หนึ่งร้อยบาทถ้วน"),
            ("one-hundred-one", 101.0, "หนึ่งร้อยเอ็ดบาทถ้วน"),
            ("one-hundred-two", 102.0, "หนึ่งร้อยสองบาทถ้วน"),
            ("one-hundred-three", 103.0, "หนึ่งร้อยสามบาทถ้วน"),
            ("one-hundred-four", 104.0, "หนึ่งร้อยสี่บาทถ้วน"),
            ("one-hundred-five", 105.0, "หนึ่งร้อยห้าบาทถ้วน"),
            ("one-hundred-six", 106.0, "หนึ่งร้อยหกบาทถ้วน"),
            ("one-hundred-seven", 107.0, "หนึ่งร้อยเจ็ดบาทถ้วน"),
            ("one-hundred-eight", 108.0, "หนึ่งร้อยแปดบาทถ้วน"),
            ("one-hundred-nine", 109.0, "หนึ่งร้อยเก้าบาทถ้วน"),
            ("one-hundred-ten", 110.0, "หนึ่งร้อยสิบบาทถ้วน"),
            ("one-hundred-eleven", 111.0, "หนึ่งร้อยสิบเอ็ดบาทถ้วน"),
            ("one-hundred-fifteen", 115.0, "หนึ่งร้อยสิบห้าบาทถ้วน"),
            ("one-hundred-twenty", 120.0, "หนึ่งร้อยยี่สิบบาทถ้วน"),
            ("one-hundred-twenty-three", 123.0, "หนึ่งร้อยยี่สิบสามบาทถ้วน"),
            ("one-hundred-twenty-five", 125.0, "หนึ่งร้อยยี่สิบห้าบาทถ้วน"),
            ("one-hundred-fifty", 150.0, "หนึ่งร้อยห้าสิบบาทถ้วน"),
            ("one-hundred-ninety-nine", 199.0, "หนึ่งร้อยเก้าสิบเก้าบาทถ้วน"),
            ("two-hundred", 200.0, "สองร้อยบาทถ้วน"),
            ("two-hundred-twenty-one", 221.0, "สองร้อยยี่สิบเอ็ดบาทถ้วน"),
            ("two-hundred-twenty-five", 225.0, "สองร้อยยี่สิบห้าบาทถ้วน"),
            ("two-hundred-fifty", 250.0, "สองร้อยห้าสิบบาทถ้วน"),
            ("two-hundred-eighty-nine", 289.0, "สองร้อยแปดสิบเก้าบาทถ้วน"),
            ("five-hundred-five", 505.0, "ห้าร้อยห้าบาทถ้วน"),
            ("seven-hundred-eighty-nine", 789.0, "เจ็ดร้อยแปดสิบเก้าบาทถ้วน"),
            ("nine-hundred-ninety-nine", 999.0, "เก้าร้อยเก้าสิบเก้าบาทถ้วน"),
            // Thousands
            ("one-thousand", 1000.0, "หนึ่งพันบาทถ้วน"),
            ("one-thousand-one", 1001.0, "หนึ่งพันเอ็ดบาทถ้วน"),
            ("one-thousand-ten", 1010.0, "หนึ่งพันสิบบาทถ้วน"),
            ("one-thousand-one-hundred", 1100.0, "หนึ่งพันหนึ่งร้อยบาทถ้วน"),
            (
                "one-thousand-one-hundred-eleven",
                1111.0,
                "หนึ่งพันหนึ่งร้อยสิบเอ็ดบาทถ้วน",
            ),
            ("two-thousand-five-hundred", 2500.0, "สองพันห้าร้อยบาทถ้วน"),
            ("five-thousand-five", 5005.0, "ห้าพันห้าบาทถ้วน"),
            (
                "nine-thousand-nine-hundred-ninety-nine",
                9999.0,
                "เก้าพันเก้าร้อยเก้าสิบเก้าบาทถ้วน",
            ),
            (
                "nine-thousand-twelve-and-thirty-four",
                9012.34,
                "เก้าพันสิบสองบาทสามสิบสี่สตางค์",
            ),
            // Thousands, Millions, and Billions
            ("ten-thousand", 10_000.0, "หนึ่งหมื่นบาทถ้วน"),
            ("one-hundred-thousand", 100_000.0, "หนึ่งแสนบาทถ้วน"),
            (
                "one-hundred-twenty-three-thousand-four-hundred-fifty-six",
                123_456.0,
                "หนึ่งแสนสองหมื่นสามพันสี่ร้อยห้าสิบหกบาทถ้วน",
            ),
            ("one-million", 1_000_000.0, "หนึ่งล้านบาทถ้วน"),
            (
                "one-million-two-hundred-thirty-four-thousand-five-hundred-sixty-seven",
                1234567.0,
                "หนึ่งล้านสองแสนสามหมื่นสี่พันห้าร้อยหกสิบเจ็ดบาทถ้วน",
            ),
            ("ten-million", 10_000_000.0, "สิบล้านบาทถ้วน"),
            ("one-hundred-million", 100_000_000.0, "หนึ่งร้อยล้านบาทถ้วน"),
            ("one-billion", 1_000_000_000.0, "หนึ่งพันล้านบาทถ้วน"),
            ("ten-billion", 10_000_000_000.0, "หนึ่งหมื่นล้านบาทถ้วน"),
            ("fifty-billion", 50_000_000_000.0, "ห้าหมื่นล้านบาทถ้วน"),
            ("one-hundred-billion", 100_000_000_000.0, "หนึ่งแสนล้านบาทถ้วน"),
            (
                "large-number",
                1_234_567_890.0,
                "หนึ่งพันสองร้อยสามสิบสี่ล้านห้าแสนหกหมื่นเจ็ดพันแปดร้อยเก้าสิบบาทถ้วน",
            ),
            // Some Trillions and Beyond
            ("one-trillion", 1_000_000_000_000.0, "หนึ่งล้านล้านบาทถ้วน"),
            (
                "very-large-number",
                1_234_567_890_123.0,
                "หนึ่งล้านสองแสนสามหมื่นสี่พันห้าร้อยหกสิบเจ็ดล้านแปดแสนเก้าหมื่นหนึ่งร้อยยี่สิบสามบาทถ้วน",
            ),
            // Floating-Point Numbers (with Satang)
            ("five-baht-exact", 5.00, "ห้าบาทถ้วน"),
            ("five-baht-twenty-five-satang", 5.25, "ห้าบาทยี่สิบห้าสตางค์"),
            ("five-baht-fifty-satang", 5.50, "ห้าบาทห้าสิบสตางค์"),
            ("five-baht-seventy-five-satang", 5.75, "ห้าบาทเจ็ดสิบห้าสตางค์"),
            (
                "fifty-one-baht-ninety-nine-satang",
                -51.994,
                "ลบห้าสิบเอ็ดบาทเก้าสิบเก้าสตางค์",
            ),
            (
                "fifty-one-baht-ninety-nine-satang",
                -51.995,
                "ลบห้าสิบสองบาทถ้วน",
            ),
            (
                "fifty-one-baht-ninety-nine-satang",
                -51.99,
                "ลบห้าสิบเอ็ดบาทเก้าสิบเก้าสตางค์",
            ),
            ("ten-baht-exact", 10.00, "สิบบาทถ้วน"),
            ("ten-baht-fifty-satang", 10.50, "สิบบาทห้าสิบสตางค์"),
            ("one-baht-one-satang", 1.01, "หนึ่งบาทหนึ่งสตางค์"),
            ("one-baht-seventy-five-satang", 1.75, "หนึ่งบาทเจ็ดสิบห้าสตางค์"),
            (
                "one-hundred-baht-fifty-satang",
                100.50,
                "หนึ่งร้อยบาทห้าสิบสตางค์",
            ),
            ("one-thousand-baht-five-satang", 1000.05, "หนึ่งพันบาทห้าสตางค์"),
            (
                "one-million-baht-one-satang",
                1000000.01,
                "หนึ่งล้านบาทหนึ่งสตางค์",
            ),
            (
                "one-thousand-two-hundred-thirty-four-baht-five-satang",
                1234.05,
                "หนึ่งพันสองร้อยสามสิบสี่บาทห้าสตางค์",
            ),
            (
                "large-float",
                123456789.25,
                "หนึ่งร้อยยี่สิบสามล้านสี่แสนห้าหมื่นหกพันเจ็ดร้อยแปดสิบเก้าบาทยี่สิบห้าสตางค์",
            ),
            // Edge Cases & Special Combinations
            ("negative-one-hundred", -100.0, "ลบหนึ่งร้อยบาทถ้วน"),
            ("ten-million-one", 10_000_001.0, "สิบล้านเอ็ดบาทถ้วน"),
            (
                "two-hundred-million-one",
                200_000_001.0,
                "สองร้อยล้านเอ็ดบาทถ้วน",
            ),
            (
                "one-billion-one-satang",
                1_000_000_000.01,
                "หนึ่งพันล้านบาทหนึ่งสตางค์",
            ),
            ("one-baht-rounded-satang", 1.234, "หนึ่งบาทยี่สิบสามสตางค์"),
            (
                "very-large-float",
                123_456_789_012.34,
                "หนึ่งแสนสองหมื่นสามพันสี่ร้อยห้าสิบหกล้านเจ็ดแสนแปดหมื่นเก้าพันสิบสองบาทสามสิบสี่สตางค์",
            ),
            (
                "very-large-total",
                870886734867267.00000,
                "แปดร้อยเจ็ดสิบล้านแปดแสนแปดหมื่นหกพันเจ็ดร้อยสามสิบสี่ล้านแปดแสนหกหมื่นเจ็ดพันสองร้อยหกสิบเจ็ดบาทถ้วน",
            ),
        ];

        for &(name, money, expect) in &cases {
            let result = words(money);

            assert_eq!(result, expect, "Fail case: {}", name);
        }
    }
}
