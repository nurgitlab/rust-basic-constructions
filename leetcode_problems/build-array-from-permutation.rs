impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let l = nums.len();
        let mut ans: Vec<i32>= Vec::with_capacity(l);

        for i in 0..l {
            ans.push(nums[nums[i] as usize]);
        }

        return ans;
    }
}