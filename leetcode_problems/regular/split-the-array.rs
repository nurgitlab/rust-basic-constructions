use std::collections::HashMap;

impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for n in nums {
            map.entry(n).and_modify(|v| *v+=1).or_insert(1);
        }

        for v in map.values() {
            if *v > 2 {
                return false;
            }
        }

        return true;
    }
}