struct Solution {}

impl Solution {
    // brutal force
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut i = 0;
        let mut result = 0;
        for num in arr {
            while result < num - 1 {
                i += 1;
                result += 1;
                if i == k {
                    return result;
                }
            }
            result += 1;
        }
        result + k - i
    }

    // binary search
    pub fn find_kth_positive1(arr: Vec<i32>, k: i32) -> i32 {
        let mut start = 0;
        let mut end = arr.len();
        while start < end {
            let mid = (start + end) / 2;
            if k > arr[mid] - (mid as i32 + 1) {
                start = mid + 1;
            } else {
                end = mid;
            }
        }
        k + start as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_kth_positive1(vec![2, 3, 4, 7, 11], 5), 9);
    assert_eq!(Solution::find_kth_positive1(vec![1, 2, 3, 4], 2), 6);
    assert_eq!(Solution::find_kth_positive1(vec![2], 1), 1);
}
