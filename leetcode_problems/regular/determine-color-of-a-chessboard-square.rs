impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let c = coordinates.chars().nth(0).unwrap();
        let n = coordinates.chars().nth(1).unwrap();

        return (c as i32 + n as i32) % 2 == 1;
    }
}