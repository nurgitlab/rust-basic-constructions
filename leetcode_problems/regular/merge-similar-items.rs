use std::collections::HashMap;

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut mem: HashMap<i32,i32> = HashMap::new();
        Solution::add_to_map(&mut mem, &items1);
        Solution::add_to_map(&mut mem, &items2);

        let mut ans: Vec<Vec<i32>> = vec![];

        for k in mem.keys() {
            let mut v: Vec<i32> = vec![];
            v.push(*k);
            v.push(mem[k]);

            ans.push(v);
        }

        ans.sort_by(|a, b| a[0].cmp(&b[0]));
        return ans;
    }

    pub fn add_to_map (map: &mut HashMap<i32,i32>, arr: &Vec<Vec<i32>>) {
        for i in 0..arr.len() {
            map.entry(arr[i][0]).and_modify(|v| *v+=arr[i][1]).or_insert(arr[i][1]);
        }
    }
}