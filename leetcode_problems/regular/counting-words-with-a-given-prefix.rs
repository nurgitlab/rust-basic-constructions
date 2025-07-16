impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let mut ans = 0;

        for i in 0..words.len() {
            if Solution::is_pref(&words[i], &pref) {
                ans+=1;
            }
        }

        return ans;
    }

    pub fn is_pref (word: &str, pref: &str) -> bool {
        if pref.len() > word.len() {
            return false;
        }

        for i in 0..pref.len() {
            let p = pref.chars().nth(i).unwrap();
            let w = word.chars().nth(i).unwrap();

            if w != p {
                return false;
            }
        }


        return true;
    }
}