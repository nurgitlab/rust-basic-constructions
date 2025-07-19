use std::collections::HashMap;
impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut mem: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            mem.entry(nums[i]).and_modify(|v| *v+=1).or_insert(1);
        }

        for k in mem.keys() {
            if mem[&k]%2 ==1 {
                return false;
            }
        }

        return true;
    }
}