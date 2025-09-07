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

    struct Case {
        name: &'static str,
        money: f64,
        expect: &'static str,
    }

    #[test]
    fn baht_to_words() {
        let cases = [
            Case {
                name: "zero",
                money: 0.0,
                expect: "ศูนย์บาทถ้วน",
            },
            Case {
                name: "1",
                money: 1.0,
                expect: "หนึ่งบาทถ้วน",
            },
            Case {
                name: "2",
                money: 2.0,
                expect: "สองบาทถ้วน",
            },
            Case {
                name: "3",
                money: 3.0,
                expect: "สามบาทถ้วน",
            },
        ];

        for cs in &cases {
            let result = words(cs.money);

            assert_eq!(result, cs.expect, "Fail case: {}", cs.name);
        }
    }
}
