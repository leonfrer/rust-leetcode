struct Solution {}

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut c = chars[0];
        let b0 = b'0';

        let mut i = 1;
        let mut counter = 1;
        let mut counter_index = 1;
        let mut dup_scale = 1;
        while i < chars.len() {
            if chars[i] == c {
                if counter == 1 {
                    // first dup char
                    counter += 1;
                    chars[i] = (b0 + 2) as char;
                    counter_index = i;
                    i += 1;
                } else {
                    counter += 1;
                    for _ in 0..=dup_scale {
                        chars.remove(counter_index);
                    }
                    // check dup_scale
                    if counter / 10_usize.pow(dup_scale as u32) != 0 {
                        dup_scale += 1;
                        i += 1;
                    }
                    let mut mc = counter;
                    while mc > 0 {
                        chars.insert(counter_index, (b0 + (mc % 10) as u8) as char);
                        mc /= 10;
                    }
                }
            } else {
                c = chars[i];
                dup_scale = 1;
                counter = 1;
                i += 1;
            }
        }

        chars.len() as i32
    }

    fn compress1(chars: &mut Vec<char>) -> i32 {
        let mut i = 0;
        while i < chars.len() {
            let mut group_len = 1;
            while group_len + i < chars.len() && chars[group_len + i] == chars[i] {
                group_len += 1;
            }
            chars.drain((i + 1)..(i + group_len));
            let mut j = 1;
            if group_len > 1 {
                while group_len > 0 {
                    chars.insert(i + 1, (b'0' + (group_len % 10) as u8) as char);
                    group_len /= 10;
                    j += 1;
                }
            }
            i += j;
        }
        chars.len() as i32
    }
}

#[test]
fn test() {
    let i = b'0';
    assert_eq!('9', (i + 9) as char);

    let mut chars1 = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    assert_eq!(Solution::compress(&mut chars1), 6);
    assert_eq!(chars1, vec!['a', '2', 'b', '2', 'c', '3']);
    let mut chars2 = vec!['a'];
    assert_eq!(Solution::compress(&mut chars2), 1);
    let mut chars3 = vec![
        'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
    ];
    assert_eq!(Solution::compress(&mut chars3), 4);
    assert_eq!(chars3, vec!['a', 'b', '1', '2']);

    let mut chars1 = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    assert_eq!(Solution::compress1(&mut chars1), 6);
    assert_eq!(chars1, vec!['a', '2', 'b', '2', 'c', '3']);
    let mut chars2 = vec!['a'];
    assert_eq!(Solution::compress1(&mut chars2), 1);
    let mut chars3 = vec![
        'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
    ];
    assert_eq!(Solution::compress1(&mut chars3), 4);
    assert_eq!(chars3, vec!['a', 'b', '1', '2']);
}
