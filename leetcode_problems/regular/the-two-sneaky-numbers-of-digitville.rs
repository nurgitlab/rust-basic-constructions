impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut mem: Vec<i32> = Vec::with_capacity(nums.len());

        mem.resize(nums.len(), 0);

        for i in 0..nums.len() {
            mem[nums[i] as usize]+=1;
        }

        let mut ans: Vec<i32> = Vec::with_capacity(nums.len());

        for i in 0..mem.len() {
            if mem[i] > 1 {
                ans.push(i as i32);
            }
        }

        return ans;
    }
}