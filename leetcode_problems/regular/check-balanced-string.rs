impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let mut c1 = 0;
        let mut c2 = 0;

        for i in 0..num.len() {
            let c = num.chars().nth(i).unwrap() as i32 - 48;

            if i % 2 == 0 {
                c1 += c;
            } else {
                c2 += c;
            }
        }
        return c1 == c2;
    }
}