impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut res = 0;
        for i in 0..sentences.len() {
            let mut ans = 1;
            let current = sentences[i].clone();
            for j in 0..current.len() {
                let c = current.chars().nth(j).unwrap();
                if c == ' ' {
                    ans+=1;
                }
            }
            if ans > res {
                res = ans;
            }
        }

        return res;
    }
}