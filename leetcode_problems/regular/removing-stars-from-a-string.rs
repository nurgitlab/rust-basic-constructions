impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut ans = String::new();
        for c in s.chars() {
            if c == '*' {
                ans.pop();
            } else {
                ans.push(c);
            }
        }

        return ans;
    }
}