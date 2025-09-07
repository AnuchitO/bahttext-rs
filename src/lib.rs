pub fn words(money: f64) -> String {
    let unit_words = [
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
    /*
           preciseAmount := math.Round(math.Abs(money)*100) / 100
        wholeBaht := math.Trunc(preciseAmount)
        satang := math.Round((preciseAmount - wholeBaht) * 100)

        bahtText := moneyToThaiWords(uint64(wholeBaht))

        if satang == 0 {
            return minus + bahtText + "บาทถ้วน"
        }
    */
    let precise_amount = (money.abs() * 100.0) / 100.0;
    let whole_baht = precise_amount.trunc() as u64;
    let satang = ((precise_amount - whole_baht as f64) * 100.0).round() as u64;

    match (whole_baht, satang) {
        (0, _) => "ศูนย์บาทถ้วน".to_string(),
        (_, _) => format!("{}บาทถ้วน", unit_words[whole_baht as usize]),
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
            ("four", 4.0, "สี่บาทถ้วน"),
            ("five", 5.0, "ห้าบาทถ้วน"),
            ("six", 6.0, "หกบาทถ้วน"),
            ("seven", 7.0, "เจ็ดบาทถ้วน"),
            ("eight", 8.0, "แปดบาทถ้วน"),
            ("nine", 9.0, "เก้าบาทถ้วน"),
        ];

        for &(name, money, expect) in &cases {
            let result = words(money);

            assert_eq!(result, expect, "Fail case: {}", name);
        }
    }
}
