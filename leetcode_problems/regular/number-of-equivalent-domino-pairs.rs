use std::collections::HashMap;
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut map:HashMap<String,i32> = HashMap::new();

        for d in dominoes {
            let mut sd = d.clone();
            sd.sort();
            let s = sd[0].to_string() + "|" + &sd[1].to_string();
            map.entry(s).and_modify(|v| *v+=1).or_insert(1);
        }

        for k in map.keys() {
            ans+=Solution::count_pairs(&map[k]);
        }
        return ans;
    }

    pub fn count_pairs(n: &i32) -> i32 {
        //1 -> 0
        //2 -> 1
        //3 -> 2 + 1
        //4 -> 3 + 2 + 1
        //5 -> 4 + 3 + 2 + 1

        //ans = (n - 1) * n / 2
        return (n - 1) * n / 2;
    }
}