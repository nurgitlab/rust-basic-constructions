impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        let mut k = s.chars().rev().collect::<String>();

        if k == s {
            return 1;
        }

        return 2;
    }
}