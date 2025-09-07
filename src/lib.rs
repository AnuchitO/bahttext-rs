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
    /*
           preciseAmount := math.Round(math.Abs(money)*100) / 100
        wholeBaht := math.Trunc(preciseAmount)
        satang := math.Round((preciseAmount - wholeBaht) * 100)

        bahtText := moneyToThaiWords(uint64(wholeBaht))

        if satang == 0 {
            return minus + bahtText + "บาทถ้วน"
        }
    */

    let total_satang = (money.abs() * 100.0) as u64;
    let whole_baht = total_satang / 100;
    let satang = total_satang % 100;

    if satang == 0 {
        format!("{}บาทถ้วน", money_to_thai_words(whole_baht))
    } else {
        format!("{}บาทถ้วน", money_to_thai_words(whole_baht))
    }
}

fn money_to_thai_words(money: u64) -> String {
    if money == 0 {
        return "ศูนย์".to_string();
    }

    let s = money.to_string();
    let len_s = s.len();

    let mut result = String::new();

    for (i, char) in s.chars().enumerate() {
        let digit = char.to_digit(10).unwrap() as usize;
        let place = len_s - i - 1;

        if digit == 0 {
            continue;
        }

        match (digit, place) {
            (2, 1) => result.push_str("ยี่สิบ"),
            (1, 1) => result.push_str("สิบ"),
            (1, 0) if !result.is_empty() => result.push_str("เอ็ด"),
            _ => {
                result.push_str(UNIT_WORDS[digit]);
                if place > 0 && place < UNIT_PLACES.len() {
                    result.push_str(UNIT_PLACES[place]);
                }
            }
        };
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

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
        ];

        for &(name, money, expect) in &cases {
            let result = words(money);

            assert_eq!(result, expect, "Fail case: {}", name);
        }
    }
}
