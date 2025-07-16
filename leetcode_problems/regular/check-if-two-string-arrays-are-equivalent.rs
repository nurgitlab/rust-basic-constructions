impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let mut w1 = String::from("");
        let mut w2 = String::from("");


        for i in 0..word1.len() {
            w1+=&word1[i];
        }

        for i in 0..word2.len() {
            w2+=&word2[i];
        }


        return w1 == w2;
    }
}