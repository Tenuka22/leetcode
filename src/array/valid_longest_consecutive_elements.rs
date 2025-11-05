use std::collections::{HashMap, HashSet};

fn longest_consecutiveOld(nums: Vec<i32>) -> i32 {
    let mut num_arr = Vec::from(nums);
    let mut seq_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut curr_longest: usize = 0;

    num_arr.sort();

    for number in num_arr.iter() {
        let prev_num = num_arr.iter().find(|curr| **curr == number - 1);

        match prev_num {
            None => {
                seq_map.insert(*number, vec![*number]);

                if &curr_longest < &1 {
                    curr_longest = 1;
                }
            }
            Some(prev) => {
                let deleted = seq_map.remove(prev);
                match deleted {
                    Some(dele) => {
                        let mut new = Vec::from(dele);
                        new.push(*number);

                        let len = &new.len();
                        seq_map.insert(*number, new);

                        if &curr_longest < len {
                            curr_longest = *len;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    curr_longest as i32
}

fn longest_consecutiveSecOld(nums: Vec<i32>) -> i32 {
    let mut num_arr = Vec::from(nums);
    let mut seq_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut curr_longest: usize = 0;

    num_arr.sort();

    for number in num_arr.iter() {
        let prev = number - 1;
        let prev_exists = num_arr.contains(&prev);

        if !prev_exists {
            seq_map.insert(*number, vec![*number]);

            if &curr_longest < &1 {
                curr_longest = 1;
            }
        } else {
            let deleted = seq_map.remove(&prev);
            match deleted {
                Some(dele) => {
                    let mut new = Vec::from(dele);
                    new.push(*number);

                    let len = &new.len();
                    seq_map.insert(*number, new);

                    if &curr_longest < len {
                        curr_longest = *len;
                    }
                }
                _ => {}
            }
        }
    }
    curr_longest as i32
}

fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let set: HashSet<i32> = nums.into_iter().collect();
    let mut curr_longest: usize = 0;

    for &number in &set {
        if !set.contains(&(number - 1)) {
            let mut current = number;
            let mut count = 1;

            while set.contains(&(current + 1)) {
                current += 1;
                count += 1;
            }
            curr_longest = curr_longest.max(count);
        }
    }
    curr_longest as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_should_be_true_1() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(longest_consecutive(nums), 4)
    }

    #[test]
    fn case_should_be_true_2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(longest_consecutive(nums), 9);
    }

    #[test]
    fn case_should_be_true_3() {
        let nums = vec![0];
        assert_eq!(longest_consecutive(nums), 1);
    }

    #[test]
    fn case_should_be_false() {
        let nums = vec![1, 0, 1, 2];
        assert_eq!(longest_consecutive(nums), 3);
    }
}
