impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut arr: Vec<char> = Vec::with_capacity(indices.len());
        arr.resize(indices.len(), '_');

        for i in 0..indices.len() {
            arr[indices[i] as usize] = s.chars().nth(i).unwrap();
        }

        let mut ans = String::from("");

        for i in 0..arr.len() {
            ans+=&arr[i].to_string();
        }

        return ans;
    }
}