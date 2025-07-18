use std::collections::HashMap;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut map = HashMap::new();

        for i in 0..paths.len() {
            let from = paths[i][0].clone();
            let to = paths[i][1].clone();

            map.insert(from, to);
        }

        for k in map.keys() {
            let value = map[k].clone();
            if map.contains_key(&value) {
                continue;
            }

            return value.to_string();
        }

        return "Error".to_string();
    }
}