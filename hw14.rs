// homeworks/hw14.rs

pub fn duplicate<T: Clone>(list: &[T]) -> Vec<T> {
    list.iter()
        .flat_map(|x| std::iter::repeat(x).take(2))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate() {
        assert_eq!(duplicate::<i32>(&[]), vec![]);
        assert_eq!(duplicate(&[1]), vec![1, 1]);
        assert_eq!(duplicate(&["a", "b"]), vec!["a", "a", "b", "b"]);
        assert_eq!(
            duplicate(&[1, 2, 3, 4, 5]),
            vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5]
        );
    }

    #[test]
    fn test_duplicate_chars() {
        assert_eq!(
            duplicate(&['a', 'b', 'c', 'c', 'd']),
            vec!['a', 'a', 'b', 'b', 'c', 'c', 'c', 'c', 'd', 'd']
        );
    }
}
