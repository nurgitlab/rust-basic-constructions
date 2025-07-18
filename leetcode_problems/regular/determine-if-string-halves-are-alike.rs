use std::collections::HashSet;
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let v: HashSet<char> = "aeiouAEIOU".to_string().chars().collect();

        let m = s.len() / 2;
        let mut ans = 0;

        for i in 0..m {
            let c = s.chars().nth(i).unwrap();

            if v.contains(&c) {
                ans+=1;
            }
        }

        for i in m..s.len() {
            let c = s.chars().nth(i).unwrap();

            if v.contains(&c) {
                ans-=1;
            }
        }

        return ans == 0;
    }
}