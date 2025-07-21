use std::collections::HashMap;
impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut mem: HashMap<i32,i32> = HashMap::new();

        for n in arr {
            mem.entry(n).and_modify(|v| *v+=1).or_insert(1);
        }

        let mut ans = -1;

        for k in mem.keys() {
            if *k == mem[k] && mem[k] > ans {
                ans = *k;
            }
        }
        return ans;
    }
}