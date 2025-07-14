impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::with_capacity(nums.len());
        
        for i in 0..n {
            ans.push(nums[i as usize]);
            ans.push(nums[(i + n)as usize]);
        }

        return ans;
    }
}