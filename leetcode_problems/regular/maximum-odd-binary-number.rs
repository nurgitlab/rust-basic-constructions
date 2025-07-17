impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut c_of_ones = 0;

        for i in 0..s.len() {
            let c = s.chars().nth(i).unwrap();

            if c == '1' {
                c_of_ones+=1;
            }
        }

        let mut c_of_zeros = s.len() - c_of_ones;
        let mut ans = String::from("");

        while c_of_ones > 0 {
            if c_of_ones > 1 {
                ans+="1";
                c_of_ones-=1;
            } else if c_of_zeros > 0 {
                ans+="0";
                c_of_zeros-=1;
            } else {
                ans+="1";
                c_of_ones-=1;
            }
        }

        return ans;
    }
}