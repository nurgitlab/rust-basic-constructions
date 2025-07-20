impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let chars: Vec<char> = sentence.chars().collect();
        
        for i in 0..chars.len() {
            if chars[i] == ' ' {
                if chars[i - 1] != chars[i + 1] {
                    return false;
                }
            }
            println!("{}", chars[i]);
        }

        return chars[0] == chars[chars.len() - 1];
    }
}