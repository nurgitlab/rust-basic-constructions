use std::collections::HashMap;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map = HashMap::new();

        for i in 1..grid.len() * grid.len() + 1 {
            map.insert(i as usize, 0);
        }

        for i in 0..grid.len() {
            for j in 0..grid.len() {
                map.entry(grid[i][j] as usize).and_modify(|v| *v += 1);
            }
        }
        println!("{:?}", map);
        let mut ans: Vec<i32> = Vec::with_capacity(2);
        ans.resize(2, 0);

        for k in map.keys() {
            if map[k] == 0 {
                ans[1] = *k as i32;
            }

            if map[k] == 2 {
                ans[0] = *k as i32;
            }
        }
        return ans;
    }
}