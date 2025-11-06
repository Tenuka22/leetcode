pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut target_sum_nums: Vec<i32> = Vec::new();

    let mut right_pointer = numbers.len() - 1;
    let mut left_pointer = 0;

    for (_, _) in numbers.iter().enumerate() {
        let right_val = numbers[right_pointer];
        let left_val = numbers[left_pointer];

        match left_val + right_val {
            sum if sum == target => {
                target_sum_nums = vec![(left_pointer + 1) as i32, (right_pointer + 1) as i32];
                break;
            }
            sum if sum > target => {
                right_pointer -= 1;
            }
            _ => {
                left_pointer += 1;
            }
        }
    }

    print!("{:?}", target_sum_nums,);

    target_sum_nums
}

#[cfg(test)]
mod tests_two_sum {
    use super::*;
    #[cfg(test)]
    mod tests_two_sum {
        use super::*;

        #[test]
        fn case_basic_example() {
            let nums = vec![2, 7, 11, 15];
            let target = 9;
            assert_eq!(two_sum(nums, target), vec![1, 2]);
        }

        #[test]
        fn case_multiple_possible_pairs() {
            let nums = vec![2, 3, 4];
            let target = 6;
            assert_eq!(two_sum(nums, target), vec![1, 3]);
        }

        #[test]
        fn case_negative_numbers() {
            let nums = vec![-1, 0];
            let target = -1;
            assert_eq!(two_sum(nums, target), vec![1, 2]);
        }

        #[test]
        fn case_no_result() {
            let nums = vec![1, 2, 3];
            let target = 100;
            assert_eq!(two_sum(nums, target), Vec::<i32>::new());
        }
    }
}
