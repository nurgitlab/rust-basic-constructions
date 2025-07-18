use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut map = HashMap::new();
        let chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() {
            if map.contains_key(&chars[i]) {
                map.entry(chars[i]).and_modify(|v| *v += 1);
            } else {
                map.insert(chars[i], 1);
            }
        }

        let mut set = HashSet::new();

        for k in map.keys() {
            let c = map[k];

            set.insert(c);
        }

        return set.len() == 1;
    }
}