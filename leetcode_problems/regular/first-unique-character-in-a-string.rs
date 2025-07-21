use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();

        for c in s.chars() {
            map.entry(c).and_modify(|v| *v+=1).or_insert(1);
        }

        let chars: Vec<char> = s.chars().collect::<Vec<char>>();
        for i in 0..chars.len() {
            if map[&chars[i]] == 1 {
                return i as i32;
            }
        }
        println!("{:?}", chars);
        return -1;
    }
}