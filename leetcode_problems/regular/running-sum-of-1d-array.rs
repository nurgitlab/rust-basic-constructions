impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];

        let mut a = 0;

        for n in nums {
            a+=n;
            ans.push(a);
        }
        return ans;
    }
}