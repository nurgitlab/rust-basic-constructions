use std::collections::HashSet;

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let alp: Vec<&str> = vec![".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."];

        let mut set = HashSet::new();
        for i in 0..words.len() {
            let word: String = words[i].clone();
            let mut res = String::from("");
            for j in 0..word.len() {
                let c = word.chars().nth(j).unwrap() as i32 - 97;

                res += alp[c as usize];
            }

            set.insert(res);
        }

        return set.len() as i32;
    }
}