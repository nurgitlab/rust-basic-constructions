use std::collections::HashSet;
use std::collections::HashMap;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut mem: HashMap<i32, i32> = HashMap::new();

        for i in 0..arr.len() {
            mem.entry(arr[i]).and_modify(|v| *v+=1).or_insert(1);
        }

        let mut set: HashSet<i32> = HashSet::new();

        for k in mem.keys() {
            if set.contains(&mem[k]) {
                return false;
            }

            set.insert(mem[k]);
        }

        return true;
    }
}