use std::collections::HashSet;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut mem1: HashSet<i32> = Solution::cr_mem(&nums1);
        let mut mem2: HashSet<i32> = Solution::cr_mem(&nums2);
        
        let mut ans1: Vec<i32> = Solution::only_in(&mem1, &mem2);
        let mut ans2: Vec<i32> = Solution::only_in(&mem2, &mem1);

        let mut ans: Vec<Vec<i32>> = vec![];
        ans.push(ans1);
        ans.push(ans2);
        return ans;
    }

    pub fn only_in (mem1: &HashSet<i32>, mem2: &HashSet<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];

        for k in mem1 {
            if !mem2.contains(&k) {
                ans.push(*k);
            }
        }

        ans.sort_by(|a, b| a.cmp(b));
        return ans;
    }

    pub fn cr_mem (arr: &Vec<i32>) -> HashSet<i32> {
        let mut s: HashSet<i32> = HashSet::new();

        for i in 0..arr.len() {
            s.insert(arr[i]);
        }

        return s;
    }
}