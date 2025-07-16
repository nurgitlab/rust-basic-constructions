impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut index_of_ch = 0;

        for i in 0..word.len() {
            let c = word.chars().nth(i).unwrap();

            if c == ch {
                index_of_ch = i;
                break;
            }
        }

        let mut ans = String::from("");

        for i in (0..index_of_ch + 1).rev() {
            let c = word.chars().nth(i).unwrap();
            ans += &c.to_string();
        }

        for i in index_of_ch + 1..word.len() {
            let c = word.chars().nth(i).unwrap();
            ans+= &c.to_string();
        }

        return ans;
    }
}