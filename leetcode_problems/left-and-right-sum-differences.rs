impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut left_sum = Vec::with_capacity(nums.len());
        let mut right_sum = Vec::with_capacity(nums.len());

        left_sum.resize(nums.len(), 0);
        right_sum.resize(nums.len(), 0);

        let mut c1 = 0;
        for i in 0..nums.len() {
            left_sum[i] = c1;
            c1 += nums[i];
        }

        let mut c2 = 0;
        // From last to first
        for i in (0..nums.len()).rev() {
            right_sum[i] = c2;
            c2 += nums[i];
        }

        for i in 0..left_sum.len() {
            // Abs in Rust is `abs()`
            left_sum[i] = (left_sum[i] - right_sum[i]).abs();
        }

        return left_sum;
    }
}