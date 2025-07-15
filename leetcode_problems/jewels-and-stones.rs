use std::collections::HashSet;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let set: HashSet<char> = jewels.chars().collect();
        let mut ans = 0;
        for i in 0..stones.len() {
            let c = stones.chars().nth(i).unwrap();

            if set.contains(&c) {
                ans+=1;
            }
        }

        return ans;
    }
}