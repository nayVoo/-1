mod homeworks;

fn main() {
    homeworks::hw03::draw_envelope();
}

// homeworks/hw03.rs

pub fn draw_envelope() {
    const WIDTH: usize = 30;
    const HEIGHT: usize = 14;
    
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if y == 0 || y == HEIGHT - 1 || x == 0 || x == WIDTH - 1 {
                // Border
                print!("*");
            } else if x == y || x == WIDTH - 1 - y {
                // Diagonal lines from top-left to bottom-right and vice versa
                print!("*");
            } else if x == HEIGHT - 1 - y || y == WIDTH - 1 - x {
                // This handles the X shape in the middle
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
