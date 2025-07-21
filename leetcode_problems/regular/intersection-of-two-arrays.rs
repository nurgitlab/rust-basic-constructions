use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set1: HashSet<i32> = nums1.into_iter().collect::<HashSet<i32>>();

        let set2: HashSet<i32> = nums2.into_iter().collect::<HashSet<i32>>();

        let mut ans: Vec<i32> = vec![];
        for i in set1 {
            if set2.contains(&i) {
                ans.push(i);
            }
        }
        return ans;
    }
}