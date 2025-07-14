impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;

        for account in accounts {
            let mut sum = 0;

            for i in 0..account.len() {
                sum+=account[i];
            }

            if sum > ans {
                ans = sum;
            }
        }

        return ans;
    }
}