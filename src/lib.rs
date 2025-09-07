pub fn words(money: f64) -> String {
    match money {
        3.0 => "สามบาทถ้วน".to_string(),
        2.0 => "สองบาทถ้วน".to_string(),
        1.0 => "หนึ่งบาทถ้วน".to_string(),
        _ => "ศูนย์บาทถ้วน".to_string(),
    }
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

        assert_eq!(got, "หนึ่งบาทถ้วน");
    }

    #[test]
    fn words_single_digit_two() {
        let got = words(2.0);

        assert_eq!(got, "สองบาทถ้วน");
    }

    #[test]
    fn words_single_digit_three() {
        let got = words(3.0);

        assert_eq!(got, "สามบาทถ้วน");
    }
}
