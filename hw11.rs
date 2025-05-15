use rand::Rng;

pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}

pub fn min_adjacent_sum(data: &[i32]) -> Option<(i32, i32, usize)> {
    data.windows(2)
        .enumerate()
        .min_by_key(|(_, pair)| pair[0] + pair[1])
        .map(|(i, pair)| (pair[0], pair[1], i))
}

pub fn print_vector_with_min_pair(vec: &[i32]) {
    if let Some((a, b, idx)) = min_adjacent_sum(vec) {
        for (i, &num) in vec.iter().enumerate() {
            if i == idx || i == idx + 1 {
                print!("[{:2}] ", num);
            } else {
                print!(" {:2}  ", num);
            }
        }
        println!("\nMin adjacent pair: {} + {} = {}", a, b, a + b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_adjacent_sum() {
        let v = vec![10, 20, 5, 15, 30];
        assert_eq!(min_adjacent_sum(&v), Some((5, 15, 2)));
    }

    #[test]
    fn test_empty_vector() {
        assert_eq!(min_adjacent_sum(&[]), None);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(min_adjacent_sum(&[10]), None);
    }
}
