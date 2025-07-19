use std::collections::HashMap;

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut mem = HashMap::new();

        Solution::add_to_map(&nums1, &mut mem);
        Solution::add_to_map(&nums2, &mut mem);

        println!("{:?}", mem);

        let mut arr: Vec<Vec<i32>> = vec![];

        for k in mem.keys() {
            let mut l: Vec<i32> = Vec::with_capacity(2);
            l.resize(2, 0);
            l[0] = *k;
            l[1] = mem[k];

            arr.push(l);
        }

        arr.sort_by(|a, b| a[0].cmp(&b[0]));
        return arr;
    }

    pub fn add_to_map(arr: &Vec<Vec<i32>>, map: &mut HashMap<i32, i32>) {
        for i in 0..arr.len() {
            let a = arr[i][0];
            let b = arr[i][1];
            if map.contains_key(&a) {
                map.entry(a).and_modify(|v| *v+=b);
            } else {
                map.insert(a, b);
            }
        }
    }
}