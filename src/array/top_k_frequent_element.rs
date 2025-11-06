use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for item in nums {
        *map.entry(item).or_insert(0) += 1;
    }

    let mut freq: Vec<(i32, i32)> = map.into_iter().collect();
    freq.sort_by(|a, b| b.1.cmp(&a.1));

    freq.into_iter()
        .take(k as usize)
        .map(|(num, _)| num)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_case1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let result = top_k_frequent(nums, 2);

        let mut sorted = result.clone();
        sorted.sort();

        assert_eq!(sorted, vec![1, 2]);
    }

    #[test]
    fn test_top_k_case2() {
        let nums = vec![1];
        let result = top_k_frequent(nums, 1);

        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_top_k_case3() {
        let nums = vec![1, 2, 1, 2, 1, 2, 3, 1, 3, 2];
        let result = top_k_frequent(nums, 2);

        let mut sorted = result.clone();
        sorted.sort();

        assert_eq!(sorted, vec![1, 2]);
    }
}
