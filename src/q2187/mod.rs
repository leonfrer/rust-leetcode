struct Solution {}

impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let min = time.iter().min().unwrap();
        let max = time.iter().max().unwrap();
        let mut end = *max as i64 * total_trips as i64;
        let mut start = total_trips as i64 * *min as i64;
        while start < end {
            let mid = (end + start) / 2;
            let trips = time.iter().fold(0, |tr, &t| mid / t as i64 + tr);
            if trips < total_trips as i64 {
                start = mid + 1;
            } else {
                end = mid;
            }
        }
        start
    }
}

#[test]
fn test() {
    // Solution::minimum_time(vec![1, 2, 3], 5);
    assert_eq!(Solution::minimum_time(vec![2], 1), 2);
}
