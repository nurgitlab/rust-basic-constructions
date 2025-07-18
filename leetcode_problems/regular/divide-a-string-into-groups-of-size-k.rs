impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let mut cloned = s.clone();

        while cloned.len() % k as usize!= 0 {
            cloned.push(fill);
        }

        let mut ans: Vec<String> = Vec::with_capacity(cloned.len() / k as usize);

        let mut i = 0;
        while i < cloned.len() {
            let mem = &cloned[i as usize..i as usize + k as usize];
            ans.push(mem.to_string());
            i+=k as usize;
        }

        return ans;
    }
}