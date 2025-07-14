impl Solution {
    pub fn transform_array(nums: Vec<i32>) -> Vec<i32> {
        let mut even = 0;

        for i in 0..nums.len() {
            if nums[i] % 2 == 0 {
                even+=1;
            }
        }

        let not_even = nums.len() - even;

        let mut ans: Vec<i32> = Vec::with_capacity(nums.len());

        ans.resize(nums.len(), 1);

        for i in 0..even {
            ans[i] = 0;
        }

        return ans
    }
}