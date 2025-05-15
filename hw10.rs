pub fn is_palindrome(num: u64) -> bool {
    if num < 10 {
        return true;
    }

    let mut n = num;
    let mut reversed = 0;

    while n > reversed {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }

    // For even and odd length palindromes
    n == reversed || n == reversed / 10
}
