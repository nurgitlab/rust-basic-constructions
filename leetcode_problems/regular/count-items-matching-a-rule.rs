impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let mut ans = 0;
        let mut ind = 0;

        if rule_key == "color" {
            ind = 1;
        }

        if rule_key == "name" {
            ind = 2;
        }

        for i in 0..items.len() {
            if items[i][ind] == rule_value {
                ans+=1;
            }
        }
        
        return ans;
    }
}