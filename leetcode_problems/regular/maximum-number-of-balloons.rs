use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        return Solution::rearrange_characters(text, "balloon".to_string());
    }

    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let chars = s.chars().collect::<Vec<char>>();
        let mut mem: HashMap<char, i32> = HashMap::new();
        let target_chars = target.chars().collect::<Vec<char>>();
        let mut target_mem: HashMap<char, i32> = HashMap::new();

        for c in chars {
            mem.entry(c).and_modify(|v| *v+=1).or_insert(1);
        }

        for c in target_chars {
            target_mem.entry(c).and_modify(|v| *v+=1).or_insert(1);
        }

        let mut ans = -1;

        for k in target_mem.keys() {
            if mem.contains_key(k) {
                let d = mem[k] / target_mem[k];
                if ans > d || ans == -1 {
                    ans = d;
                }
            } else {
                ans = 0;
                break;
            }
        }
        return ans;    
    }
}