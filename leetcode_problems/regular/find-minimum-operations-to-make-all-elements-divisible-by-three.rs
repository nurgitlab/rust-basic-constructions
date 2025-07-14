impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        for i in 0..nums.len() {
            if nums[i] % 3 != 0 {
                ans+=1;
            }
        }

        return ans;
    }
}