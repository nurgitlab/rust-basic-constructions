use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut mem: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            mem.entry(nums[i]).and_modify(|v| *v+=1).or_insert(1);
        }

        let mut arr: Vec<Vec<i32>> = vec![];
        for k in mem.keys() {
            let mut m: Vec<i32> = vec![];

            m.push(*k);
            m.push(mem[&k]);
            arr.push(m);
        }

        arr.sort_by(|a, b| a[1].cmp(&b[1]).then(b[0].cmp(&a[0])));
        println!("{:?}", arr);
        let mut ans: Vec<i32> = vec![];

        for i in 0..arr.len() {
            for j in 0..arr[i][1] {
                ans.push(arr[i][0]);
            }
        }
        return ans;
    }
}