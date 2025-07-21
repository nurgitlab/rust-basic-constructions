use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let set: HashSet<i32> = nums.clone().into_iter().collect::<HashSet<i32>>();

        return set.len() != nums.len();
    }
}