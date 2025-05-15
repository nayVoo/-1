// homeworks/hw05.rs

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    // Handle the case where one number is 0
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    // Using Euclid's algorithm
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(18, 48), 6);
        assert_eq!(gcd(17, 5), 1);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(252, 105), 21);
        assert_eq!(gcd(1071, 462), 21);
    }
}



mod homeworks;

fn main() {
    println!("GCD of 48 and 18 is: {}", homeworks::hw05::gcd(48, 18));
    println!("GCD of 1071 and 462 is: {}", homeworks::hw05::gcd(1071, 462));
}
