impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut v: Vec<&str> = s.split(" ").collect();

        let splited: Vec<&str> = s.split(" ").collect();
        let mut res: Vec<&str> = Vec::with_capacity(splited.len());
        res.resize(splited.len(), "");

        for i in 0..v.len() {

            let n = v[i].chars().nth(v[i].len() - 1).unwrap();

            let local_str = &v[i][0..v[i].len() - 1];
            println!("{}-{}", local_str, n);
            res[n as usize - 49] = local_str;
        }

        return res.join(" ");
    }
}