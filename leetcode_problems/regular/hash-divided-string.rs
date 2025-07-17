impl Solution {
    pub fn string_hash(s: String, k: i32) -> String {
        let mut sl = &s;
        let mut ans = String::from("");

        let mut i = 0;
        while i < s.len() {
            let mut sum: u8 = 0;
            for j in i..i+k as usize {
                let c = s.chars().nth(j).unwrap();
                sum+=c as u8 - 97;
                sum%=26;
            }

            ans+= &((sum + 97) as char).to_string();
            i+=k as usize;
        }

        return ans;
    }
}