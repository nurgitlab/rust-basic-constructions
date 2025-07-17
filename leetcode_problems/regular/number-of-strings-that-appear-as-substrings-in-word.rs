impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        let mut ans = 0;

        for i in 0..patterns.len() {
            let mut w: &str = &word.clone();
            let mut splitted: Vec<&str> = w.split(&patterns[i]).collect();

            if splitted.len() > 1 {
                ans+=1;
            }
        }

        return ans;
    }
}