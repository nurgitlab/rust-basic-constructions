impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut splited: Vec<&str> = s.split("|").collect();

        let mut i = 0;
        let mut ans = 0;
        while i < splited.len() {
            for j in 0..splited[i].len() {
                let c = splited[i].chars().nth(j).unwrap();

                if c == '*' {
                    ans+=1;
                }
            }
            i+=2;
        }

        return ans;
    }
}