use std::collections::HashMap;

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut mem: HashMap<i32,i32> = HashMap::new();

        for i in 0..digits.len() {
            mem.entry(digits[i]).and_modify(|v| *v+=1).or_insert(1);
        }

        let mut ans:  Vec<i32> = vec![];

        for i in 100..1000 {
            let s = i.to_string();

            let mut m:HashMap<i32,i32>= HashMap::new();

            for j in 0..s.len() {
                let c = s.chars().nth(j).unwrap() as i32 - 48;
                m.entry(c).and_modify(|v| *v+=1).or_insert(1);
            }

            if Solution::is_can(&m, &mem) && i % 2 == 0 {
                ans.push(i as i32);
            }
        }

        return ans;
    }

    pub fn is_can(m1: &HashMap<i32,i32>, m2: &HashMap<i32,i32>) -> bool {
        for k in m1.keys() {
            if !m2.contains_key(k) {
                return false;
            }

            if m2[k] < m1[k] {
                return false;
            }
        }

        return true;
    }
}