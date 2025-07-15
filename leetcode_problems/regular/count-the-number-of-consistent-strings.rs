use std::collections::HashSet;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut set: HashSet<char> = allowed.chars().collect();

        let mut ans = 0;

        for i in 0..words.len() {
            let mut isOk: bool = true;
            for j in 0..words[i].len() {
                let c = words[i].chars().nth(j).unwrap();
                if !set.contains(&c) {
                    isOk = false;
                    break;
                }
            }

            if isOk {
                ans+=1;
            }
        }

        return ans;
    }
}