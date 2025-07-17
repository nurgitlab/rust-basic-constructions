impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let mut m_w: Vec<String> = Vec::with_capacity(words.len());

        for i in 0..words.len() {
            m_w.push(Solution::sort_string(words[i].clone()));
        }

        let mut ans = 0;

        for i in 0..m_w.len() {
            for j in i + 1..m_w.len() {
                if m_w[i] == m_w[j] {
                    ans+=1;
                }
            }
        }

        return ans;
    }

    pub fn sort_string (word: String) -> String {
        let mut arr: Vec<char> = Vec::with_capacity(word.len());

        for i in 0..word.len() {
            let c = word.chars().nth(i).unwrap();
            arr.push(c);
        }

        arr.sort_by(|a, b| a.cmp(b));

        let mut ans = String::from("");
        for i in 0..arr.len() {
            ans+=&arr[i].to_string();
        }

        return ans;
    }
}