impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut sorted_points = points.clone();
        sorted_points.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut ans = 0;

        for i in 0..sorted_points.len() - 1 {
            if sorted_points[i + 1][0] - sorted_points[i][0] > ans {
                ans = sorted_points[i + 1][0] - sorted_points[i][0];
            }
        }
        //println!("{:?}", sorted_points);

        return ans;
    }
}