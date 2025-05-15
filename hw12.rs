use rand::Rng;

// Функція для підрахунку мінімальної кількості переміщень
pub fn count_permutations(shipments: &[u32]) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;
    
    if total % n != 0 {
        return None;
    }
    
    let target = total / n;
    let mut moves = 0;
    let mut current_sum = 0;
    
    for (i, &shipment) in shipments.iter().enumerate() {
        current_sum += shipment;
        let expected_sum = target * (i as u32 + 1);
        if current_sum != expected_sum {
            moves += 1;
        }
    }
    
    Some(moves)
}

// Генерація коректного набору даних
pub fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let target = rng.gen_range(10..=100);
    let mut shipments = vec![target; n];
    
    // Додаємо деяку варіативність
    for i in 0..n/2 {
        let idx = rng.gen_range(0..n);
        let delta = rng.gen_range(1..=5);
        if shipments[idx] >= delta + 2 {
            shipments[idx] -= delta;
            shipments[(idx + 1) % n] += delta;
        }
    }
    
    shipments
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_possible_distribution() {
        let shipments = vec![10, 20, 30];
        assert_eq!(count_permutations(&shipments), None);
        
        let shipments = vec![15, 15, 15];
        assert_eq!(count_permutations(&shipments), Some(0));
        
        let shipments = vec![10, 20, 15, 15];
        assert_eq!(count_permutations(&shipments), Some(1));
    }
    
    #[test]
    fn test_generation() {
        let shipments = gen_shipments(5);
        assert!(count_permutations(&shipments).is_some());
    }
}
