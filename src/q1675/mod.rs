struct Solution {}

use std::collections::BinaryHeap;
impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::with_capacity(nums.len());
        let mut min = i32::MAX;
        let mut answer = i32::MAX;
        for num in nums.into_iter() {
            let n = if num & 1 == 1 { num * 2 } else { num };
            min = min.min(n);
            heap.push(n);
        }
        while let Some(num) = heap.pop() {
            answer = answer.min(num - min);
            if num & 1 == 1 {
                break;
            }
            let n = num / 2;
            min = min.min(n);
            heap.push(n);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::minimum_deviation(vec![1, 2, 3, 4]), 1);
        assert_eq!(Solution::minimum_deviation(vec![4, 1, 5, 20, 3]), 3);
        assert_eq!(Solution::minimum_deviation(vec![2, 10, 8]), 3);
    }
}
