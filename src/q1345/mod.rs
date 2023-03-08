struct Solution {}

use std::collections::HashMap;
use std::collections::VecDeque;
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        if arr.len() == 1 {
            return 0;
        }
        let last_index = arr.len() - 1;
        let mut result = 0;
        let mut groups: HashMap<i32, Vec<usize>> = HashMap::new();

        for (i, &num) in arr.iter().enumerate() {
            groups.entry(num).or_insert_with(Vec::new).push(i);
        }

        let mut queue: VecDeque<Option<usize>> = VecDeque::new();
        queue.push_back(Some(0));
        queue.push_back(None);
        while let Some(ele) = queue.pop_front() {
            if let Some(index) = ele {
                if index == last_index {
                    break;
                }
                if index > 1 {
                    if let Some(index_vec) = groups.get_mut(&arr[index - 1]) {
                        for (i, value) in index_vec.iter().enumerate() {
                            if *value == index - 1 {
                                index_vec.remove(i);
                                queue.push_back(Some(index - 1));
                                break;
                            }
                        }
                    }
                }
                if index < last_index {
                    if let Some(index_vec) = groups.get_mut(&arr[index + 1]) {
                        for (i, value) in index_vec.iter().enumerate() {
                            if *value == index + 1 {
                                index_vec.remove(i);
                                queue.push_back(Some(index + 1));
                                break;
                            }
                        }
                    }
                }
                if let Some(index_vec) = groups.get(&arr[index]) {
                    for i in index_vec {
                        queue.push_back(Some(*i));
                    }
                }
                groups.remove(&arr[index]);
            } else {
                queue.push_back(None);
                result += 1;
            }
        }

        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_jumps(vec![100,-23,-23,404,100,23,23,23,3,404]), 3);
    assert_eq!(Solution::min_jumps(vec![7]), 0);
    assert_eq!(Solution::min_jumps(vec![7,6,9,6,9,6,9,7]), 1);
}
