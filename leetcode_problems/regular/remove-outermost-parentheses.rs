impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut k = 0;

        let mut ans = String::from("");

        for c in s.chars() {
            if c == '(' {
                k+=1;
            } else {
                k-=1;
            }

            if k > 1 && c == '(' {
                ans+=&c.to_string();
            } else if c == ')' && k > 0 {
                ans+=&c.to_string();
            }
        }

        return ans;
    }
}