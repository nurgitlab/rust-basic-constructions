use std::collections::HashSet;

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut ans = 0;

        for i in 0..words.len() {
            for j in i+1..words.len() {
                if Solution::is_similar(&words[i], &words[j]) {
                    ans+=1;
                }
            }
        }

        return ans;
    }

    pub fn is_similar (w1: &String, w2: &String) -> bool {
        let s1: HashSet<char> = w1.chars().collect();
        let s2: HashSet<char> = w2.chars().collect();

        for k in &s1 {
            if !s2.contains(&k) {
                return false;
            }
        }

        for k in &s2 {
            if !s1.contains(&k) {
                return false;
            }
        }

        return true;
    }
}