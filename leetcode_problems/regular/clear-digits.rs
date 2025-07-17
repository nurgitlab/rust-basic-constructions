impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut mem: Vec<String> = Vec::new();

        for i in 0..s.len() {
            let c = s.chars().nth(i).unwrap();

            if c >= '0' && c <= '9' {
                mem.pop();
            } else {
                mem.push(c.to_string());
            }
        }

        return mem.join("").to_string();
    }
}