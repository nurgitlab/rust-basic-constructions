impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        let mut ans = 0;

        for i in 0..strs.len() {
            let mut m = 0;
            if Solution::is_contain_num(&strs[i]) {
                println!("{}", strs[i]);
                m = strs[i].parse::<i32>().unwrap();
            } else {
                m = strs[i].len() as i32;
            }

            if ans < m {
                ans = m;
            }
        }

        return ans;
    }

    pub fn is_contain_num(s: &String) -> bool {
        let mut d = 0;
        for i in 0..s.len() {
            let c = s.chars().nth(i).unwrap() as i32 - 48;

            if c >=0 && c <= 9 {
                d+=1;
            }
        }

        return s.len() == d;
    }
}