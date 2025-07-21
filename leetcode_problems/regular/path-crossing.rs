use std::collections::HashSet;
impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        let mut set: HashSet<String> = HashSet::new();
        let chars = path.chars().collect::<Vec<char>>();
        set.insert("0|0".to_string());
        for c in chars {
            if c == 'N' {
                x+=1;
            }

            if c == 'S' {
                x-=1;
            }

            if c == 'E' {
                y-=1;
            }

            if c == 'W' {
                y+=1;
            }

            let current = x.to_string() + "|" + &y.to_string(); 
            if set.contains(&current) {
                return true;
            }

            set.insert(current);
        }

        return false;
    }
}