use std::collections::HashMap;

impl Solution {
    pub fn check_prime_frequency(nums: Vec<i32>) -> bool {
        let mut mem: HashMap<i32,i32> = HashMap::new();

        for i in 0..nums.len() {
            mem.entry(nums[i]).and_modify(|v| *v+=1).or_insert(1);
        }

        for k in mem.keys() {
            if Solution::is_prime(&mem[k]) {
                return true;
            }
        }

        return false;
    }

    pub fn is_prime (n: &i32) -> bool {
        for i in 2..*n {
            if *n % i == 0 {
                return false;
            }
        }

        return *n > 1;
    }
}