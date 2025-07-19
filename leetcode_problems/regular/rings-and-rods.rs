use std::collections::HashSet;

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut arr: Vec<HashSet<char>> = Vec::with_capacity(10);
        arr.resize(10, HashSet::new());

        let mut i = 0;
        while i < rings.len() {
            let c = rings.chars().nth(i).unwrap();
            let n = rings.chars().nth(i + 1).unwrap() as usize - 48;

            arr[n].insert(c);
            i+=2;
        }

        let mut ans = 0;

        for i in 0..arr.len() {
            if arr[i].len() == 3 {
                ans+=1;
            }
        }

        return ans;
    }
}