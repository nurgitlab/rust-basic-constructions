use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map1 = Solution::to_map(&nums1);
        let mut map2 = Solution::to_map(&nums2);

        let mut ans: Vec<i32> = vec![];

        for k in map1.keys() {
            if !map2.contains_key(k) {
                continue;
            }

            let mut min = map1[k];
            if map2[k] < map1[k] {
                min = map2[k];
            }

            for j in 0..min {
                ans.push(*k);
            }
        }
        println!("{:?}", map1);

        return ans;
    }

    pub fn to_map (arr: &Vec<i32>) -> HashMap<i32, i32> {
        let mut ans: HashMap<i32, i32> = HashMap::new();
        for n in arr {
            ans.entry(*n).and_modify(|v| *v+=1).or_insert(1);
        }

        return ans;
    }
}