impl Solution {
    pub fn is_contain(word: &String, x: char) -> bool {
        let l = word.len();

        for i in 0..l {
            let localChar = word.chars().nth(i).unwrap();
            if localChar == x {
                return true
            }
        }

        return false
    }

    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let count_words = words.len();
        let mut ans: Vec<i32>= Vec::with_capacity(count_words);

        for i in 0..count_words {
            if Solution::is_contain(&words[i], x) {
                ans.push(i as i32);
            }
        }

        return ans
    }
}