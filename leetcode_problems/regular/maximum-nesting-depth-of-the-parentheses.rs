impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut max = 0;
        let mut counter = 0;

        for i in 0..s.len() {
            let c = s.chars().nth(i).unwrap();

            if c == '(' {
                counter+=1;
            } else if c == ')' {
                counter-=1;
            }

            if max < counter {
                max = counter;
            }
        }

        return max;
    }
}