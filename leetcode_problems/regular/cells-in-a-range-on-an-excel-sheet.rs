impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let l1 = s.chars().nth(0).unwrap() as u8;
        let l2 = s.chars().nth(3).unwrap() as u8;

        let n1 = s.chars().nth(1).unwrap() as u8;
        let n2 = s.chars().nth(4).unwrap() as u8;

        let mut ans: Vec<String> = vec![];

        for i in l1..l2 + 1 {
            for j in n1..n2 + 1 {
                let mut local = String::from("");
                local+=&(i as char).to_string();
                local+=&(j as char).to_string();

                ans.push(local);
            }
        }

        return ans;
    }
}