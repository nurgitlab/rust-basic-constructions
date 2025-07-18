impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let mut max = 0;
        let mut l = 0;

        for i in 0..s.len() {
            let c = s.chars().nth(i).unwrap();

            if c == 'E' {
                l+=1;
            } else {
                l-=1;
            }

            if l > max {
                max = l;
            }
        }

        return max;
    }
}