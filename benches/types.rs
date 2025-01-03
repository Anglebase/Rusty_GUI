#[cfg(test)]
mod tests {
    use rusty_gui::*;

    #[test]
    fn test_point_add() {
        assert_eq!(pos!(1, 2) + pos!(3, 4), pos!(4, 6));
    }

    #[test]
    fn test_point_sub() {
        assert_eq!(pos!(5, 5) - pos!(2, 3), pos!(3, 2));
    }

    #[test]
    fn test_point_mul() {
        assert_eq!(pos!(10, 10) * 2.0, pos!(20, 20));
        assert_eq!(pos!(10, 10) * 0.5, pos!(5, 5));
    }

    #[test]
    fn test_point_div() {
        assert_eq!(pos!(20, 20) / 2.0, pos!(10, 10));
        assert_eq!(pos!(10, 10) / 0.5, pos!(20, 20));
    }

    #[test]
    fn test_size_add() {
        assert_eq!(size!(1, 2) + size!(3, 4), size!(4, 6));
    }

    #[test]
    fn test_size_sub() {
        assert_eq!(size!(5, 5) - size!(2, 3), size!(3, 2));
    }

    #[test]
    fn test_size_mul() {
        assert_eq!(size!(10, 10) * 2.0, size!(20, 20));
        assert_eq!(size!(10, 10) * 0.5, size!(5, 5));
    }

    #[test]
    fn test_size_div() {
        assert_eq!(size!(20, 20) / 2.0, size!(10, 10));
        assert_eq!(size!(10, 10) / 0.5, size!(20, 20));
    }

    #[test]
    fn test_rect_contains_happy_path() {
        let rect1 = rect!(0, 0, 10, 10);
        let rect2 = rect!(2, 2, 6, 6);
        assert!(rect1.contains(&rect2));
    }

    #[test]
    fn test_rect_contains_edge_case_no_overlap() {
        let rect1 = rect!(0, 0, 5, 5);
        let rect2 = rect!(6, 6, 5, 5);
        assert!(!rect1.contains(&rect2));
    }

    #[test]
    fn test_rect_contains_edge_case_partial_overlap() {
        let rect1 = rect!(0, 0, 5, 5);
        let rect2 = rect!(4, 4, 5, 5);
        assert!(!rect1.contains(&rect2));
    }

    #[test]
    fn test_rect_contains_edge_case_same_size() {
        let rect1 = rect!(0, 0, 5, 5);
        let rect2 = rect!(0, 0, 5, 5);
        assert!(rect1.contains(&rect2));
    }

    #[test]
    fn test_rect_contains_edge_case_larger_rect() {
        let rect1 = rect!(0, 0, 5, 5);
        let rect2 = rect!(0, 0, 10, 10);
        assert!(!rect1.contains(&rect2));
    }

    #[test]
    fn test_rect_bitand_happy_path() {
        let rect1 = rect!(0, 0, 10, 10);
        let rect2 = rect!(5, 5, 10, 10);
        assert_eq!(rect1 & rect2, Some(rect!(5, 5, 5, 5)));
    }

    #[test]
    fn test_rect_bitand_edge_case_no_overlap() {
        let rect1 = rect!(0, 0, 5, 5);
        let rect2 = rect!(6, 6, 5, 5);
        assert_eq!(rect1 & rect2, None);
    }

    #[test]
    fn test_rect_bitand_edge_case_partial_overlap() {
        let rect1 = rect!(0, 0, 5, 5);
        let rect2 = rect!(4, 4, 5, 5);
        assert_eq!(rect1 & rect2, Some(rect!(4, 4, 1, 1)));
    }

    #[test]
    fn test_rect_bitand_edge_case_same_rect() {
        let rect1 = rect!(0, 0, 5, 5);
        let rect2 = rect!(0, 0, 5, 5);
        assert_eq!(rect1 & rect2, Some(rect!(0, 0, 5, 5)));
    }

    #[test]
    fn test_rect_bitand_edge_case_one_inside_other() {
        let rect1 = rect!(0, 0, 10, 10);
        let rect2 = rect!(2, 2, 5, 5);
        assert_eq!(rect1 & rect2, Some(rect!(2, 2, 5, 5)));
    }

    #[test]
    fn test_rect_bitor_happy_path() {
        let rect1 = rect!(0, 0, 5, 5);
        let rect2 = rect!(5, 5, 5, 5);
        assert_eq!(rect1 | rect2, rect!(0, 0, 10, 10));
    }

    #[test]
    fn test_rect_bitor_edge_case_no_overlap() {
        let rect1 = rect!(0, 0, 5, 5);
        let rect2 = rect!(6, 6, 5, 5);
        assert_eq!(rect1 | rect2, rect!(0, 0, 11, 11));
    }

    #[test]
    fn test_rect_bitor_edge_case_partial_overlap() {
        let rect1 = rect!(0, 0, 5, 5);
        let rect2 = rect!(4, 4, 5, 5);
        assert_eq!(rect1 | rect2, rect!(0, 0, 9, 9));
    }

    #[test]
    fn test_rect_bitor_edge_case_same_rect() {
        let rect1 = rect!(0, 0, 5, 5);
        let rect2 = rect!(0, 0, 5, 5);
        assert_eq!(rect1 | rect2, rect!(0, 0, 5, 5));
    }

    #[test]
    fn test_rect_bitor_edge_case_one_inside_other() {
        let rect1 = rect!(0, 0, 10, 10);
        let rect2 = rect!(2, 2, 5, 5);
        assert_eq!(rect1 | rect2, rect!(0, 0, 10, 10));
    }

    #[test]
    fn test_distance_happy_path() {
        let point1 = pos!(0, 0);
        let point2 = pos!(3, 4);
        assert_eq!(point1.distance(&point2), 5.0);
    }

    #[test]
    fn test_distance_same_point() {
        let point1 = pos!(5, 5);
        let point2 = pos!(5, 5);
        assert_eq!(point1.distance(&point2), 0.0);
    }

    #[test]
    fn test_distance_negative_coordinates() {
        let point1 = pos!(-3, -4);
        let point2 = pos!(0, 0);
        assert_eq!(point1.distance(&point2), 5.0);
    }

    #[test]
    fn test_distance_mixed_coordinates() {
        let point1 = pos!(-3, 4);
        let point2 = pos!(3, -4);
        assert_eq!(point1.distance(&point2), 10.0);
    }

    #[test]
    fn test_distance_zero_width() {
        let point1 = pos!(0, 0);
        let point2 = pos!(0, 3);
        assert_eq!(point1.distance(&point2), 3.0);
    }

    #[test]
    fn test_distance_zero_height() {
        let point1 = pos!(0, 0);
        let point2 = pos!(4, 0);
        assert_eq!(point1.distance(&point2), 4.0);
    }
}
