use std::collections::HashSet;

type Point = (i32, i32);

pub fn calculate_occupied_area(rectangles: &[(Point, Point)]) -> u64 {
    let mut occupied = HashSet::new();
    
    for &((x1, y1), (x2, y2)) in rectangles {
        for x in x1.min(x2)..x1.max(x2) {
            for y in y1.min(y2)..y1.max(y2) {
                occupied.insert((x, y));
            }
        }
    }
    
    occupied.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_rectangle() {
        let rects = [((0, 0), (2, 2))];
        assert_eq!(calculate_occupied_area(&rects), 4);
    }

    #[test]
    fn test_overlapping_rectangles() {
        let rects = [((0, 0), (2, 2)), ((1, 1), (3, 3))];
        assert_eq!(calculate_occupied_area(&rects), 7);
    }

    #[test]
    fn test_non_overlapping_rectangles() {
        let rects = [((0, 0), (1, 1)), ((2, 2), (3, 3))];
        assert_eq!(calculate_occupied_area(&rects), 2);
    }

    #[test]
    fn test_empty_input() {
        let rects = [];
        assert_eq!(calculate_occupied_area(&rects), 0);
    }
}
