struct Solution {}

use std::collections::VecDeque;

impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut visit: Vec<Vec<bool>> = vec![vec![false; m]; n];
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 0 && !visit[i][j] && Self::bfs(i, j, &mut visit, n, m, &grid) {
                    ans += 1;
                }
            }
        }
        ans
    }

    fn bfs(row: usize, col: usize, visit: &mut Vec<Vec<bool>>, n: usize, m: usize, grid: &Vec<Vec<i32>>) -> bool {
        visit[row][col] = true;
        let mut queue = VecDeque::new();
        queue.push_back((row, col));
        let mut is_close = true;
        let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        while let Some((r, c)) = queue.pop_front() {
            for (i, j) in directions.iter() {
                let ri = r as i32 + i;
                let cj = c as i32 + j;
                if ri < 0 || ri >= n as i32 || cj < 0 || cj >= m as i32 {
                    is_close = false;
                } else if grid[ri as usize][cj as usize] == 0 && !visit[ri as usize][cj as usize] {
                    queue.push_back((ri as usize, cj as usize));
                    visit[ri as usize][cj as usize] = true;
                }
            }
        }
        is_close
    }
}

#[test]
fn test() {
    let grid = vec![vec![1,1,1,1,1,1,1,0],vec![1,0,0,0,0,1,1,0],vec![1,0,1,0,1,1,1,0],vec![1,0,0,0,0,1,0,1],vec![1,1,1,1,1,1,1,0]];
    Solution::closed_island(grid);
}