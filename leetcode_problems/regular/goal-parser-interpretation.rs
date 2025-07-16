impl Solution {
    pub fn interpret(command: String) -> String {
        let mut ans: Vec<char> = Vec::with_capacity(10);

        for i in 0..command.len() {
            let c1: char = command.chars().nth(i).unwrap();
            let mut c2: char = '_';

            if i + 1 < command.len() {
                c2 = command.chars().nth(i + 1).unwrap();
            }

            if c1 == '(' && c2 == ')' {
                ans.push('o');
                continue;
            }

            if c1 == '(' && c2 != ')' {
                continue;
            }

            if c1 == ')' {
                continue;
            }

            ans.push(c1);
        }

        return ans.iter().collect();
    }
}