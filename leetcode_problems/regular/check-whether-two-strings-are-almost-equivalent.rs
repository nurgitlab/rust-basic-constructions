use std::collections::HashMap;

impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let mut mem: HashMap<char, i32> = HashMap::new();

        let ch1 = word1.chars().collect::<Vec<char>>();
        let ch2 = word2.chars().collect::<Vec<char>>();

        for c in ch1 {
            mem.entry(c).and_modify(|v| *v+=1).or_insert(1);
        }

        for c in ch2 {
            mem.entry(c).and_modify(|v| *v-=1).or_insert(-1);
        }

        for k in mem.keys() {
            if mem[k].abs() > 3 {
                return false;
            }
        }

        return true;
    }
}