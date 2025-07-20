use std::collections::HashMap;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();

        for &n in &nums {
            map.entry(n).and_modify(|v| *v+=1).or_insert(1);
        }

        for k in map.keys() {
            if map[k] == nums.len() / 2 {
                return *k;
            }
        }

        return -1;
    }
}