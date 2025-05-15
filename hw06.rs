// homeworks/hw06.rs

pub fn draw_christmas_tree(triangles: usize) {
    // Calculate total height (each triangle is 2 rows + base)
    let height = triangles * 2 + 1;
    
    // Draw each row
    for row in 0..height {
        // Determine which triangle we're in
        let triangle_num = (row / 2).min(triangles - 1);
        let width = 2 * triangle_num + 1;
        
        // Calculate spaces and stars for this row
        let spaces = if row % 2 == 0 {
            height - 1 - triangle_num
        } else {
            height - 1 - triangle_num - 1
        };
        
        let stars = if row % 2 == 0 {
            1
        } else {
            width + 2
        };
        
        // Print the row
        (0..spaces).for_each(|_| print!(" "));
        (0..stars).for_each(|_| print!("*"));
        println!();
    }
    
    // Draw trunk
    (0..height - 2).for_each(|_| print!(" "));
    println!("*");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        //



mod homeworks;

fn main() {
    homeworks::hw06::draw_christmas_tree(3); // Draws tree with 3 triangles
}
