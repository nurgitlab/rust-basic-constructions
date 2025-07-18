impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        let mut ans = String::from("");

        if n % 2 == 0 {
            ans+=&Solution::cr_s(n - 1, 'a');
            ans+=&'b'.to_string();
        } else {
            ans+=&Solution::cr_s(n, 'a');
        }

        return ans;
    }

    pub fn cr_s (n: i32, c: char) -> String {
        let mut ans = String::from("");

        for i in 0..n {
            ans+=&c.to_string();
        }

        return ans;
    }
}