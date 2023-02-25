struct Solution {}

use std::collections::BinaryHeap;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        if len < 2 {
            return 0;
        }
        let mut heap = BinaryHeap::with_capacity(len);
        let mut max = i32::MIN;
        for i in (0..(len - 1)).rev() {
            heap.push(prices[i + 1]);
            if let Some(&price) = heap.peek() {
                let earn = price - prices[i];
                if earn > 0 {
                    max = max.max(earn);
                }
            }
        }
        if max > 0 {
            max
        } else {
            0
        }
    }

    pub fn max_profit1(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut min_price = i32::MAX;
        for price in prices {
            if max < price - min_price {
                max = price - min_price;
            }
            if price < min_price {
                min_price = price;
            }
        }
        max
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    assert_eq!(Solution::max_profit(vec![1, 2]), 1);
    assert_eq!(Solution::max_profit1(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit1(vec![7, 6, 4, 3, 1]), 0);
}
