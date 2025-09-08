use bahttext::words;

#[test]
fn test_words_integration() {
    // Test basic number conversions
    assert_eq!(words(0.0), "ศูนย์บาทถ้วน");
    assert_eq!(words(1.0), "หนึ่งบาทถ้วน");
    assert_eq!(words(10.0), "สิบบาทถ้วน");
    assert_eq!(words(11.0), "สิบเอ็ดบาทถ้วน");
    assert_eq!(words(21.0), "ยี่สิบเอ็ดบาทถ้วน");

    // Test with decimal points
    assert_eq!(words(0.50), "ศูนย์บาทห้าสิบสตางค์");
    assert_eq!(words(1.25), "หนึ่งบาทยี่สิบห้าสตางค์");
    assert_eq!(words(100.75), "หนึ่งร้อยบาทเจ็ดสิบห้าสตางค์");

    // Test with large numbers
    assert_eq!(words(1_000.0), "หนึ่งพันบาทถ้วน");
    assert_eq!(words(1_234.56), "หนึ่งพันสองร้อยสามสิบสี่บาทห้าสิบหกสตางค์");
    assert_eq!(
        words(1_234_567.89),
        "หนึ่งล้านสองแสนสามหมื่นสี่พันห้าร้อยหกสิบเจ็ดบาทแปดสิบเก้าสตางค์"
    );

    // Test with very small amounts
    assert_eq!(words(0.01), "ศูนย์บาทหนึ่งสตางค์");
    assert_eq!(words(0.10), "ศูนย์บาทสิบสตางค์");
    assert_eq!(words(0.11), "ศูนย์บาทสิบเอ็ดสตางค์");
}

#[test]
fn test_words_large_numbers() {
    assert_eq!(words(1_000_000.0), "หนึ่งล้านบาทถ้วน");
    assert_eq!(
        words(1_000_001.0),
        "หนึ่งล้านเอ็ดบาทถ้วน"
    );
    assert_eq!(
        words(1_234_567_890.0),
        "หนึ่งพันสองร้อยสามสิบสี่ล้านห้าแสนหกหมื่นเจ็ดพันแปดร้อยเก้าสิบบาทถ้วน"
    );
    assert_eq!(
        words(1_000_000_000_000.0),
        "หนึ่งล้านล้านบาทถ้วน"
    );
}

#[test]
fn test_words_negative_numbers() {
    assert_eq!(words(-1.0), "ลบหนึ่งบาทถ้วน");
    assert_eq!(words(-100.50), "ลบหนึ่งร้อยบาทห้าสิบสตางค์");
    assert_eq!(words(-1_234.56), "ลบหนึ่งพันสองร้อยสามสิบสี่บาทห้าสิบหกสตางค์");
}

#[test]
fn test_words_edge_cases() {
    // Test with exactly 0.5 satang (should round up to 1 satang)
    assert_eq!(words(0.005), "ศูนย์บาทหนึ่งสตางค์");

    // Test with 0.49 (should round down to 49 satang)
    assert_eq!(words(0.49), "ศูนย์บาทสี่สิบเก้าสตางค์");

    // Test with 0.50 (should round up to 50 satang)
    assert_eq!(words(0.50), "ศูนย์บาทห้าสิบสตางค์");

    // Test with very large number
    assert_eq!(
        words(9_999_999_999_999.99),
        "เก้าล้านเก้าแสนเก้าหมื่นเก้าพันเก้าร้อยเก้าสิบเก้าล้านเก้าแสนเก้าหมื่นเก้าพันเก้าร้อยเก้าสิบเก้าบาทเก้าสิบเก้าสตางค์"
    );
}
