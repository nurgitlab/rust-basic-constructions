impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut ans = s.clone();

        while ans.len() > 2 {
            let mut m = String::from("");

            for i in 0..ans.len() - 1 {
                let c1 = ans.chars().nth(i).unwrap() as i32 - 48;
                let c2 = ans.chars().nth(i + 1).unwrap() as i32 - 48;
                m+=&((c1 + c2) % 10).to_string();
            }

            ans=m.clone();
        }

        let v: Vec<char> = ans.chars().collect();
        return v[0] == v[1];
    }
}