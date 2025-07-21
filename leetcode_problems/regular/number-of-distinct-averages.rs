use std::collections::HashSet;

impl Solution {
    pub fn distinct_averages(nums: Vec<i32>) -> i32 {
        let mut set:HashSet<i32> = HashSet::new();
        let mut sorted = nums.clone();
        sorted.sort();

        for i in 0..sorted.len() / 2 {
            let min = sorted[i];
            let max = sorted[sorted.len() - i - 1];

            set.insert(min + max);
        }
        return set.len() as i32;
    }
}