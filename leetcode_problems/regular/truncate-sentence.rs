impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut ans = String::from("");
        let mut counter = k;
        for i in 0..s.len() {
            let c = s.chars().nth(i).unwrap();

            if c == ' ' {
                counter-=1;
            }

            if counter == 0 {
                break;
            }

            ans+=&c.to_string();
        }

        return ans;
    }
}