// homeworks/hw07.rs

pub fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];

        data.iter().for_each(|(a, b)| {
            assert_eq!(invert_the_case(a.to_string()), b.to_string());
            assert_eq!(invert_the_case(b.to_string()), a.to_string());
        });
    }

    #[test]
    fn test_mixed_case() {
        assert_eq!(invert_the_case("RustПрограммирование".to_string()), "rUSTпРОГРАММИРОВАНИЕ");
        assert_eq!(invert_the_case("123!@#".to_string()), "123!@#");
    }
}
