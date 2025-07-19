use std::collections::HashMap;
impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut map = HashMap::new();

        for i in 0..names.len() {
            map.insert(heights[i], names[i].clone());
        }

        let mut ans: Vec<String> = vec![];

        let mut h = heights.clone();
        h.sort_by(|a, b| b.cmp(a));

        println!("{:?}", map);
        println!("{:?}", h);

        for i in 0..h.len() {
            let c = map[&h[i]].clone();
            ans.push(c);
        }
        
        return ans;
    }
}