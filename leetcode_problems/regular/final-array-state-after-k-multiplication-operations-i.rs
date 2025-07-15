impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut counter = k;
        let mut my_nums = nums.clone();

        while counter > 0 {
            let mut min_index = 0;
            let mut min = 0;
            for i in 0..my_nums.len() {
                if i == 0 {
                    min = my_nums[0];
                }

                if min > my_nums[i] {
                    min_index = i;
                    min = my_nums[i];
                }
            }

            my_nums[min_index] *= multiplier;
            counter-=1;
        }

        return my_nums;
    }
}