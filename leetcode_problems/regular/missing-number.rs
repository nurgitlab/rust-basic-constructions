use std::collections::HashMap;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut mem: HashMap<i32,i32> = HashMap::new();

        for i in 0..nums.len() {
            mem.insert(i as i32, 0);
        }

        for n in &nums {
            mem.entry(*n).and_modify(|v| *v+=1).or_insert(1);
        }
        println!("{:?}", mem);

        for k in mem.keys() {
            if mem[k] == 0 {
                return *k;
            }
        }

        return nums.len() as i32;
    }
}