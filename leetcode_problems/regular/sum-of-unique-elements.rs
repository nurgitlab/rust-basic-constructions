use std::collections::HashMap;

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut mem:HashMap<i32,i32> = HashMap::new();

        for i in 0..nums.len() {
            mem.entry(nums[i]).and_modify(|v| *v+=1).or_insert(1);
        }

        let mut ans = 0;

        for k in mem.keys() {
            if mem[k] == 1 {
                ans+=k;
            }
        }

        return ans;
    }
}