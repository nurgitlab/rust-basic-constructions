impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut arr: Vec<char> = Vec::with_capacity(s.len());
        arr.resize(s.len(), 'a');

        for i in 0..s.len() / 2 + 1 {
            let c1 = s.chars().nth(i).unwrap();
            let c2 = s.chars().nth(s.len() - 1 - i).unwrap();

            if c1 < c2 {
                arr[i] = c1;
                arr[s.len() - 1 - i] = c1;
            } else {
                arr[i] = c2;
                arr[s.len() - 1 - i] = c2;
            }
        }

        let mut ans = String::from("");
        for i in 0..arr.len() {
            ans+=&arr[i].to_string();
        }

        return ans;
    }
}