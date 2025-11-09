pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;

    let mut max_area = 0;

    while left < right {
        let h = height[left].min(height[right]);
        let width = (right - left) as i32;

        max_area = max_area.max(h * width);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area_sample_1() {
        let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = max_area(input);
        assert_eq!(result, 49);
    }

    #[test]
    fn test_max_area_sample_2() {
        let input = vec![1, 1];
        let result = max_area(input);
        assert_eq!(result, 1);
    }
}
