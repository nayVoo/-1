// homeworks/hw15.rs

use std::collections::HashSet;

pub fn solve_muxa_slon() {
    let mut solutions = Vec::new();
    
    for m in 1..=9 {
        for u in 0..=9 {
            if u == m { continue; }
            for x in 0..=9 {
                if x == m || x == u { continue; }
                for a in 1..=9 {
                    if a == m || a == u || a == x { continue; }
                    for s in 1..=9 {
                        if s == m || s == u || s == x || s == a { continue; }
                        for l in 0..=9 {
                            if l == m || l == u || l == x || l == a || l == s { continue; }
                            for o in 0..=9 {
                                if o == m || o == u || o == x || o == a || o == s || o == l { continue; }
                                for n in 0..=9 {
                                    if n == m || n == u || n == x || n == a || n == s || n == l || n == o { continue; }
                                    
                                    let muxa = m * 1000 + u * 100 + x * 10 + a;
                                    let slon = s * 1000 + l * 100 + o * 10 + n;
                                    
                                    if muxa * a == slon {
                                        solutions.push((m, u, x, a, s, l, o, n));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("Знайдено {} розв'язків:", solutions.len());
    for (i, sol) in solutions.iter().enumerate() {
        let (m, u, x, a, s, l, o, n) = sol;
        println!("\nРозв'язок #{}:", i+1);
        println!("  {}{}{}{}", m, u, x, a);
        println!("×       {}", a);
        println!("  ------");
        println!("    {}{}{}{}", s, l, o, n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        // Запускаємо функцію, щоб перевірити, що вона не падає
        solve_muxa_slon();
    }
}
