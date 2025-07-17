impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        let mut ans = 0;

        //11, 12

        for i in 0..details.len() {
            let c1 = details[i].chars().nth(11).unwrap() as i32 - 48;
            let c2 = details[i].chars().nth(12).unwrap() as i32 - 48;

            if c1 * 10 + c2 > 60 {
                ans+=1;
            }
        }

        return ans;
    }
}