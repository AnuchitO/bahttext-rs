#![no_main]

use bahttext::words;
use libfuzzer_sys::fuzz_target;
use arbitrary::{Arbitrary, Unstructured};

#[derive(Debug, arbitrary::Arbitrary)]
struct FuzzInput {
    // Use a u64 to represent f64 bits for better fuzzing
    bits: u64,
    // Test different magnitudes
    magnitude: i8,
    // Test with different decimal precisions
    decimal_places: u8,
}

// Helper function to generate edge cases
fn get_edge_cases() -> Vec<f64> {
    vec![
        // Common values
        0.0, 1.0, -1.0, 10.0, 100.0, 1_000.0, 1_000_000.0,
        // Decimal values
        0.01, 0.1, 0.25, 0.5, 0.75, 0.99,
        // Large numbers
        1e15, -1e15, 1e20, -1e20,
        // Special float values
        f64::INFINITY, f64::NEG_INFINITY, f64::NAN,
        // Subnormal numbers
        f64::MIN_POSITIVE, -f64::MIN_POSITIVE,
        // Boundary values
        f64::MAX, f64::MIN,
    ]
}

fuzz_target!(|data: &[u8]| {
    // Test with edge cases first
    for &value in &get_edge_cases() {
        let _ = words(value);
    }

    // Skip if input is too small
    if data.len() < 10 {
        return;
    }

    let mut u = Unstructured::new(data);

    // Test with random f64 values
    if let Ok(input) = FuzzInput::arbitrary(&mut u) {
        // Create a value with controlled magnitude
        let base_value = f64::from_bits(input.bits);
        let magnitude = input.magnitude as i32 % 38; // Limit to reasonable exponent
        let scaled_value = if magnitude >= 0 {
            base_value * 10.0f64.powi(magnitude)
        } else {
            base_value / 10.0f64.powi(-magnitude)
        };

        // Test with different decimal precisions
        let decimal_places = input.decimal_places % 10; // Limit to 0-9 decimal places
        let factor = 10.0f64.powi(decimal_places as i32);
        let precise_value = (scaled_value * factor).round() / factor;

        // Test with the generated value
        let _ = words(precise_value);

        // Also test with the negative value
        if precise_value != 0.0 {
            let _ = words(-precise_value);
        }
    }

    // Test with some specific patterns
    if data.len() >= 8 {
        // Test with exact byte patterns
        for chunk in data.chunks_exact(8) {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(chunk);
            let value = f64::from_le_bytes(bytes);
            let _ = words(value);
        }
    }
});
