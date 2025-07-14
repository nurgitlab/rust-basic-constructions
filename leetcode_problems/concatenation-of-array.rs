impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let l = nums.len();
        let mut ans: Vec<i32> = Vec::with_capacity(l * 2);
        ans.resize(l * 2, 0);

        for i in 0..l {
            ans[i] = nums[i];
            ans[i + l] = nums[i];
        }

        return ans;
    }
}