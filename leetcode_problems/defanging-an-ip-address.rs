//Correct and short solution
impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.replace(".", "[.]")
    }
}


//MySolution
impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut ans: Vec<char> = Vec::with_capacity(10);

        for i in 0..address.len() {
            let ch = address.chars().nth(i).unwrap();
            if (ch == '.') {
                ans.push('[');
                ans.push('.');
                ans.push(']');
            } else {
                ans.push(ch);
            }
        }

        
        return ans.iter().collect();
    }
}