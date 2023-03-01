struct Solution {}

use rand::{thread_rng, Rng};
impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        fn quick_sort(nums: &mut [i32]) {
            if nums.len() < 2 {
                return;
            }

            let mut rng = thread_rng();
            let r: usize = rng.gen_range(0..nums.len());
            nums.swap(0, r);

            let last = nums.len() - 1;
            let pivot = nums[last];
            let mut mark = 0;
            for i in 0..last {
                if nums[i] < pivot {
                    nums.swap(i, mark);
                    mark += 1;
                }
            }
            nums.swap(mark, last);
            quick_sort(&mut nums[0..mark]);
            quick_sort(&mut nums[(mark + 1)..]);
        }
        quick_sort(&mut nums);
        nums
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
    assert_eq!(
        Solution::sort_array(vec![5, 1, 1, 2, 0, 0]),
        vec![0, 0, 1, 1, 2, 5]
    );
    assert_eq!(
        Solution::sort_array(vec![-4, 0, 7, 4, 9, -5, -1, 0, -7, -1]),
        vec![-7, -5, -4, -1, -1, 0, 0, 4, 7, 9]
    );
}
