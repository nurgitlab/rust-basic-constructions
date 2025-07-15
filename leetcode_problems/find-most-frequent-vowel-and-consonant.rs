use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut set = HashSet::new();
        let vowel = vec!['a', 'e', 'i', 'o', 'u'];

        for i in 0..vowel.len() {
            set.insert(vowel[i]);
        }

        let mut map = HashMap::new();
        for i in 0..s.len() {
            let c = s.chars().nth(i).unwrap();
            
            if !map.contains_key(&c) {
                map.insert(c, 0);
            }

            map.entry(c).and_modify(|v| *v += 1);
        }

        for i in 0..vowel.len() {
            if !map.contains_key(&vowel[i]) {
                map.insert(vowel[i], 0);
            }
        }

        let mut max_vowel = 0;
        let mut max_other = 0;

        for (k, v) in &map {
            if set.contains(&k) {
                if *v > max_vowel {
                    max_vowel = *v;
                }
            } else {
                if *v > max_other {
                    max_other = *v;
                }
            }
        }

        return max_vowel + max_other;
    }
}