use std::collections::HashMap;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
      let mut sorted: Vec<i32> = arr.clone();
      sorted.sort();

      let mut rank: Vec<i32> = vec![];

      let mut i = 0;
      let mut j = 0;
      while i < sorted.len() {
        rank.push(j as i32);
        while i + 1 < sorted.len() && sorted[i] == sorted[i + 1] {
            rank.push(j as i32);
            i+=1;
        }
        i+=1;
        j+=1;
      }

      let mut map: HashMap<i32, i32> = HashMap::new();
      for i in 0..rank.len() {
        map.insert(sorted[i], rank[i]);
      }

      let mut ans = vec![];
      for n in arr {
        ans.push(map[&n] + 1);
      }

      return ans;  
    }
}