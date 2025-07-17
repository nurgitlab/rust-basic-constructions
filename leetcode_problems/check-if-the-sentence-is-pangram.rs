use std::collections::HashMap;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut map = HashMap::new();

        for i in 0..sentence.len() {
            let c = sentence.chars().nth(i).unwrap();
            map.insert(c, 1);
        }

        return map.keys().len() == 26;
    }
}