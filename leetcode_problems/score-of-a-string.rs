impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut ans = 0;

        for i in 1..s.len() {
            let c2 = s.chars().nth(i).unwrap();
            let c1 = s.chars().nth(i - 1).unwrap();

            ans+=(c1 as i32 - c2 as i32).abs();
        }

        return ans;
    }
}