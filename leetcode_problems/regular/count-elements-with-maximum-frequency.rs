use std::collections::HashMap;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut mem: HashMap<i32,i32> = HashMap::new();

        for n in nums {
            mem.entry(n).and_modify(|v| *v+=1).or_insert(1);
        }

        let mut max = 0;
        let mut c = 0;

        for k in mem.keys() {
            if mem[k] > max {
                max = mem[k];
                c = 1;
            } else if mem[k] == max {
                c+=1;
            }
        }

        return c * max;
    }
}