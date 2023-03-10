struct Solution {}

impl Solution {
    // sliding window
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let h_len = haystack.len();
        let n_len = needle.len();
        if h_len == 0 {
            return -1;
        }
        if n_len == 0 {
            return 0;
        }
        if n_len > h_len {
            return -1;
        }
        let max = h_len - n_len;
        let mut i = 0;
        let h_chars: Vec<char> = haystack.chars().collect();
        let n_chars: Vec<char> = needle.chars().collect();
        while i <= max {
            if n_chars[0] != h_chars[i] {
                i += 1;
                while i <= max && n_chars[0] != h_chars[i] {
                    i += 1;
                }
            }

            if i <= max {
                let mut j = i + 1;
                let mut k = 1;
                let end = i + n_len;
                while j < end && h_chars[j] == n_chars[k] {
                    j += 1;
                    k += 1;
                }
                if j == end {
                    return i as i32;
                }
            }
            i += 1;
        }
        -1
    }

    // Knuth-Morris-Pratt Algorithm (KMP)
    fn str_str1(haystack: String, needle: String) -> i32 {
        let m = needle.len();
        if m > haystack.len() {
            return -1;
        }
        let m_chars: Vec<char> = needle.chars().collect();
        let mut lps = vec![0; m];
        let mut prev = 0;
        let mut i = 1;
        while i < m {
            if m_chars[i] == m_chars[prev] {
                lps[i] = prev + 1;
                prev += 1;
                i += 1;
            } else if prev == 0 {
                lps[i] = 0;
                i += 1;
            } else {
                prev = lps[prev - 1];
            }
        }

        let mut i = 0;
        let mut j = 0;
        let n_chars: Vec<char> = haystack.chars().collect();
        while i < haystack.len() {
            if n_chars[i] == m_chars[j] {
                i += 1;
                j += 1;
                if j == m {
                    return (i - m) as i32;
                }
            } else if j == 0 {
                i += 1;
            } else {
                j = lps[j - 1];
            }
        }
        -1
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::str_str1(String::from("sadbutsad"), String::from("sad")),
        0
    );
    assert_eq!(
        Solution::str_str1(String::from("leetcode"), String::from("leeto")),
        -1
    );
    assert_eq!(
        Solution::str_str1(String::from("hello"), String::from("ll")),
        2
    );
    assert_eq!(
        Solution::str_str1(String::from("aaa"), String::from("aaaa")),
        -1
    );
}
