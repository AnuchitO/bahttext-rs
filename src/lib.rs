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
    fn baht_to_words() {
        let cases = [
            ("zero", 0.0, "ศูนย์บาทถ้วน"),
            ("one", 1.0, "หนึ่งบาทถ้วน"),
            ("two", 2.0, "สองบาทถ้วน"),
            ("three", 3.0, "สามบาทถ้วน"),
        ];

        for &(name, money, expect) in &cases {
            let result = words(money);

            assert_eq!(result, expect, "Fail case: {}", name);
        }
    }
}
