impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        let mut ans = String::from("");

        for i in 0..words.len() {
            ans+=&words[i].chars().nth(0).unwrap().to_string();
        }

        return ans == s;
    }
}