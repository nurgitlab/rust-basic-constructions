impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut x = 0;
        let mut y = 0;

        for c in moves.chars() {
            if c == 'U' {
                x+=1;
            }

            if c == 'D' {
                x-=1;
            }

            if c == 'R' {
                y+=1;
            }

            if c == 'L' {
                y-=1;
            }
        }

        return x == 0 && y == 0;
    }
}