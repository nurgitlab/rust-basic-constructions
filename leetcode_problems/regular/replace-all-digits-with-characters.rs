impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut ans = String::from("");

        for i in 0..s.len() {
            let c = s.chars().nth(i).unwrap();

            if c >= 'a' && c <= 'z' {
                ans+= &c.to_string();
                continue;
            }

            let n = c as i32 - 48;
            let prev_c = s.chars().nth(i - 1).unwrap();
            let new_char = ((prev_c as u8) + n as u8) as char;
            println!("{}-{}", c, new_char);
            ans+= &new_char.to_string();
        }
        return ans;
    }
}