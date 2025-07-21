use std::collections::HashSet;
use std::collections::HashMap;

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut set1: HashSet<i32> = nums1.into_iter().collect::<HashSet<i32>>();
        let mut set2: HashSet<i32> = nums2.into_iter().collect::<HashSet<i32>>();
        let mut set3: HashSet<i32> = nums3.into_iter().collect::<HashSet<i32>>();

        let mut mem: HashMap<i32,i32> = HashMap::new();
        Solution::add_to_map(&mut mem, &set1);
        Solution::add_to_map(&mut mem, &set2);
        Solution::add_to_map(&mut mem, &set3);

        let mut ans: Vec<i32> = vec![];

        for k in mem.keys() {
            if mem[k] > 1 {
                ans.push(*k);
            }
        }
        return ans;
    }

    pub fn add_to_map (map: &mut HashMap<i32, i32>, set: &HashSet<i32>) {
        for i in set {
            map.entry(*i).and_modify(|v| *v+=1).or_insert(1);
        }
    }
}