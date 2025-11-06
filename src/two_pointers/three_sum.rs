pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut result = Vec::new();

    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];

            match sum.cmp(&0) {
                std::cmp::Ordering::Equal => {
                    result.push(vec![nums[i], nums[left], nums[right]]);

                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }

                    left += 1;
                    right -= 1;
                }
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Greater => right -= 1,
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_triplets_sum_to_zero(triplets: Vec<Vec<i32>>) -> bool {
        triplets.iter().all(|t| t.iter().sum::<i32>() == 0)
    }

    #[test]
    fn test_three_sum_case1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let result = three_sum(nums);

        assert!(all_triplets_sum_to_zero(result));
    }

    #[test]
    fn test_three_sum_case2() {
        let nums = vec![0, 1, 1];
        let result = three_sum(nums);

        assert!(all_triplets_sum_to_zero(result));
    }

    #[test]
    fn test_three_sum_case3() {
        let nums = vec![2, -3, 0, -2, -5, -5, -4, 1, 2, -2, 2, 0, 2, -4, 5, 5, -10];
        let result = three_sum(nums);

        assert!(all_triplets_sum_to_zero(result));
    }
}
