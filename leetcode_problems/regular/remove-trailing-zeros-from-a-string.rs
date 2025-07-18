impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {

        let mut j = num.len() - 1;
        for i in (0..num.len()).rev() {
            let c = num.chars().nth(i).unwrap();

            if c == '0' {
                j-=1;
            } else {
                break;
            }
        }
        println!("{}={}", j, num.len());

        let mut ans = String::from("");
        for i in 0..j + 1 {
            let c = num.chars().nth(i).unwrap();
            ans+=&c.to_string();
        }
        return ans;
    }
}