use std::collections::HashMap;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for i in 1..nums.len() + 1 {
            map.insert(i as i32, 0);
        }

        for n in nums {
            map.entry(n).and_modify(|v| *v+=1);
        }

        let mut ans: Vec<i32> = vec![];

        for k in map.keys() {
            if map[k] == 0 {
                ans.push(*k);
            }
        }

        return ans;
    }
}