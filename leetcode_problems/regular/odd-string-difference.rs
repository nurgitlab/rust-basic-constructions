impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let mut m = words.clone();
        
        for i in 0..words.len() {
            m[i] = Solution::to_base(&words[i]);
        }

        for i in 1..m.len() - 1 {
            if m[i] != m[i + 1] && m[i] != m[i - 1] {
                return words[i].clone();
            }
        }

        if m[0] == m[1] {
            return words[m.len() - 1].clone();
        }

        return words[0].clone();
    }

    pub fn to_base(word: &String) -> String {
        let mut ans = String::from("");
        let base = word.chars().nth(0).unwrap() as u8;

        for c in word.chars() {
            let n = c as u8 - base;
            ans+=&(n as char).to_string();
        }

        return ans;
    }
}