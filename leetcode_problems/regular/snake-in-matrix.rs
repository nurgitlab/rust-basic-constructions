impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
      let mut matrix: Vec<Vec<i32>> = Vec::new();

      for i in 0..n {
        let mut line: Vec<i32> = Vec::new();
        let k = i * n;

        for j in 0..n {
            line.push(k + j);
        }
        matrix.push(line);
      }

      let mut i = 0;
      let mut j = 0;

      for k in 0..commands.len() {
        if commands[k] == "UP" {
            i-=1;
        }

        if commands[k] == "DOWN" {
            i+=1;
        }

        if commands[k] == "RIGHT" {
            j+=1;
        }

        if commands[k] == "LEFT" {
            j-=1;
        }
      }

      return matrix[i][j];  
    }
}