impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        for a in 0..nums.len() {
            for b in a + 1..nums.len() {
                for c in b + 1..nums.len() {
                    for d in c + 1..nums.len() {
                        if nums[a] + nums[b] + nums[c] == nums[d] {
                            ans+=1;
                        }
                    }
                }
            }
        }

        return ans;
    }
}