use std::collections::HashSet;
use std::collections::VecDeque;
struct Solution {}
impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        queue.push_back(Some((sr, sc)));
        let rl = image.len();
        let cl = image[0].len();
        queue.push_back(None);
        let four_direction = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let old_color = image[sr as usize][sc as usize];
        let mut set = HashSet::new();
        set.insert((sr, sc));
        while let Some(o) = queue.pop_front() {
            if let Some((r, c)) = o {
                for (rp, cp) in four_direction.into_iter() {
                    let dr = r + rp;
                    let dc = c + cp;
                    if dr < rl as i32
                        && dr >= 0
                        && dc < cl as i32
                        && dc >= 0
                        && image[dr as usize][dc as usize] == old_color
                        && !set.contains(&(dr, dc))
                    {
                        image[dr as usize][dc as usize] = color;
                        queue.push_back(Some((dr, dc)));
                        println!("pushed: {:?}", (dr, dc));
                        set.insert((dr, dc));
                    }
                }
            } else {
                if queue.is_empty() {
                    break;
                } else {
                    queue.push_back(None);
                }
            }
        }
        image
    }

    pub fn flood_fill1(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        fn dfs(im: &mut Vec<Vec<i32>>, r: usize, c: usize, color: &i32, new_color: &i32) {
            if *color == im[r][c] {
                im[r][c] = *new_color;
                if r >= 1 {
                    dfs(im, r - 1, c, color, new_color);
                }
                if c >= 1 {
                    dfs(im, r, c - 1, color, new_color);
                }
                if r + 1 < im.len() {
                    dfs(im, r + 1, c, color, new_color);
                }
                if c + 1 < im[0].len() {
                    dfs(im, r, c + 1, color, new_color);
                }
            }
        }
        let old_color = image[sr as usize][sc as usize];
        if color != old_color {
            dfs(&mut image, sr as usize, sc as usize, &old_color, &color);
        }
        image
    }
}

#[test]
fn test() {
    Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2);
}
