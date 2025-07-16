impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut ans = 0;
        let mut d = 0;

        for i in 0..s.len() {
            let c = s.chars().nth(i).unwrap();

            if c == 'R' {
                d+=1;
            } else {
                d-=1;
            }

            if d == 0 {
                ans+=1;
            }
        }

        return ans;
    }
}