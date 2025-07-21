use std::collections::HashMap;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let map1 = Solution::to_map(&s);
        let map2 = Solution::to_map(&t);

        for k in map1.keys() {
            if !map2.contains_key(k) || map2[k] != map1[k] {
                return *k;
            }
        }

        for k in map2.keys() {
            if !map1.contains_key(k) || map2[k] != map1[k] {
                return *k;
            }
        }
        return 'a';
    }

    pub fn to_map (s: &String) -> HashMap<char, i32> {
        let mut map: HashMap<char, i32> = HashMap::new();

        let chars: Vec<char> = s.chars().collect::<Vec<char>>();

        for c in chars {
            map.entry(c).and_modify(|v| *v+=1).or_insert(1);
        }

        return map;
    }
}