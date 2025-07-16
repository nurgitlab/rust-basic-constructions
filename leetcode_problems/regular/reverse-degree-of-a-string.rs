impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        let mut ans = 0;

        for i in 0..s.len() {
            let c = s.chars().nth(i).unwrap();
            ans+= Solution::chat_to_int(c) * (i + 1);

            println!("{}-{}", c, c as usize)
        }

        return ans as i32;
    }

    pub fn chat_to_int (c: char) -> usize {
        let n = (c as i32) - 97;
        let r = (n - 26).abs();

        return r as usize;
    }
}