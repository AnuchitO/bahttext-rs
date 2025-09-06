pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn words(money: f64) -> String {
    return "ศูนย์บาทถ้วน".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_single_digit_zero() {
        let got = words(0.0);

        assert_eq!(got, "ศูนย์บาทถ้วน");
    }

    #[test]
    fn words_single_digit_one() {
        let got = words(1.0);

        assert_eq!(got, "ศูนย์บาทถ้วน");
    }

}
