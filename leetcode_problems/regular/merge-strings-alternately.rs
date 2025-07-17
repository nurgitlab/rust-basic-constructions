impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut ans = String::from("");

        let mut i = 0;
        let mut j = 0;

        while i < word1.len() || j < word2.len() {
            if i < word1.len() {
                ans+=&word1.chars().nth(i).unwrap().to_string();
            }

            if j < word2.len() {
                ans+=&word2.chars().nth(j).unwrap().to_string();
            }

            i+=1;
            j+=1;
        }

        return ans;
    }
}