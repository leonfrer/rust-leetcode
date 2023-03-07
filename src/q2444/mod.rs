struct Solution {}

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let n = nums.len();
        let mut left = -1;
        let mut last_min = -1;
        let mut last_max = -1;
        let mut result = 0;

        for i in 0..n {
            if min_k <= nums[i] && nums[i] <= max_k {
                if nums[i] == min_k { last_min = i as i32; }
                if nums[i] == max_k { last_max = i as i32; }
                result += 0.max(last_min.min(last_max) - left) as i64;
            } else {
                left = i as i32;
                last_min = -1;
                last_max = -1;
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_subarrays(vec![1,3,5,2,7,5], 1, 5), 2);
    assert_eq!(Solution::count_subarrays(vec![1,1,1,1], 1, 1), 10);
}