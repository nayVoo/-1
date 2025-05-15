// homeworks/hw04.rs

pub fn draw_diamond() {
    const SIZE: usize = 6; // Controls the size of the diamond
    
    // Upper half of the diamond
    for i in 0..SIZE {
        for j in 0..(2 * SIZE - 1) {
            if j >= (SIZE - 1 - i) && j <= (SIZE - 1 + i) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    
    // Lower half of the diamond
    for i in (0..SIZE-1).rev() {
        for j in 0..(2 * SIZE - 1) {
            if j >= (SIZE - 1 - i) && j <= (SIZE - 1 + i) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
