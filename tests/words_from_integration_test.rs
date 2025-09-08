use bahttext::words_from;

#[test]
fn test_words_from_integration() {
    // Test basic number conversions
    assert_eq!(words_from("0").unwrap(), "ศูนย์บาทถ้วน");
    assert_eq!(words_from("1").unwrap(), "หนึ่งบาทถ้วน");
    assert_eq!(words_from("10").unwrap(), "สิบบาทถ้วน");
    assert_eq!(words_from("11").unwrap(), "สิบเอ็ดบาทถ้วน");
    assert_eq!(words_from("21").unwrap(), "ยี่สิบเอ็ดบาทถ้วน");

    // Test with decimal points
    assert_eq!(words_from("0.50").unwrap(), "ศูนย์บาทห้าสิบสตางค์");
    assert_eq!(words_from("1.25").unwrap(), "หนึ่งบาทยี่สิบห้าสตางค์");
    assert_eq!(words_from("100.75").unwrap(), "หนึ่งร้อยบาทเจ็ดสิบห้าสตางค์");

    // Test with thousand separators
    assert_eq!(words_from("1,000").unwrap(), "หนึ่งพันบาทถ้วน");
    assert_eq!(words_from("1,234.56").unwrap(), "หนึ่งพันสองร้อยสามสิบสี่บาทห้าสิบหกสตางค์");
    assert_eq!(
        words_from("1,234,567.89").unwrap(),
        "หนึ่งล้านสองแสนสามหมื่นสี่พันห้าร้อยหกสิบเจ็ดบาทแปดสิบเก้าสตางค์"
    );

    // Test with whitespace
    assert_eq!(words_from(" 1234.56 ").unwrap(), "หนึ่งพันสองร้อยสามสิบสี่บาทห้าสิบหกสตางค์");

    // Test error cases
    assert!(words_from("abc").is_err());
    assert!(words_from("1.2.3").is_err());
    assert!(words_from("").is_err());
    assert!(words_from(" ").is_err());
}

#[test]
fn test_large_numbers() {
    assert_eq!(words_from("1000000").unwrap(), "หนึ่งล้านบาทถ้วน");
    assert_eq!(
        words_from("1000001").unwrap(),
        "หนึ่งล้านเอ็ดบาทถ้วน"
    );
    assert_eq!(
        words_from("1234567890").unwrap(),
        "หนึ่งพันสองร้อยสามสิบสี่ล้านห้าแสนหกหมื่นเจ็ดพันแปดร้อยเก้าสิบบาทถ้วน"
    );
    assert_eq!(
        words_from("1000000000000").unwrap(),
        "หนึ่งล้านล้านบาทถ้วน"
    );
}

#[test]
fn test_negative_numbers() {
    assert_eq!(words_from("-1").unwrap(), "ลบหนึ่งบาทถ้วน");
    assert_eq!(words_from("-100.50").unwrap(), "ลบหนึ่งร้อยบาทห้าสิบสตางค์");
    assert_eq!(words_from("-1,234.56").unwrap(), "ลบหนึ่งพันสองร้อยสามสิบสี่บาทห้าสิบหกสตางค์");
}
