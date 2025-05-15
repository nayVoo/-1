// homeworks/hw09.rs

pub fn rotate(s: String, n: isize) -> String {
    if s.is_empty() {
        return s;
    }

    let len = s.chars().count() as isize;
    let normalized_shift = ((n % len) + len) % len;
    let split_point = (len - normalized_shift) as usize;

    let (left, right) = s.split_at(split_point);
    format!("{}{}", right, left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0, "abcdefgh"),
            (8, "abcdefgh"),
            (-8, "abcdefgh"),
            (1, "habcdefg"),
            (2, "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10, "cdefghab"),
        ];

        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(rotate(s.clone(), *n), exp.to_string());
        });
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(rotate("".to_string(), 5), "".to_string());
    }

    #[test]
    fn test_single_char() {
        assert_eq!(rotate("a".to_string(), 5), "a".to_string());
        assert_eq!(rotate("a".to_string(), -3), "a".to_string());
    }
}
