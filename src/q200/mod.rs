struct Solution {}

use std::collections::VecDeque;
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut islands = 0;
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == '1' {
                    islands += 1;
                    Self::bfs(&mut grid, &mut queue, row, col, rows, cols);
                }
            }
        }
        islands
    }

    fn bfs(
        grid: &mut Vec<Vec<char>>,
        queue: &mut VecDeque<(usize, usize)>,
        row: usize,
        col: usize,
        rows: usize,
        cols: usize,
    ) {
        queue.push_back((row, col));
        grid[row][col] = '0';
        while let Some((i, j)) = queue.pop_front() {
            for (x, y) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let xi = x + i as i32;
                let yj = y + j as i32;
                if xi >= 0
                    && (xi as usize) < rows
                    && yj >= 0
                    && (yj as usize) < cols
                    && grid[xi as usize][yj as usize] == '1'
                {
                    grid[xi as usize][yj as usize] = '0';
                    queue.push_back((xi as usize, yj as usize));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let case1 = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(Solution::num_islands(case1), 1);
        let case2 = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(Solution::num_islands(case2), 3);
        let case3 = vec![vec!['1'], vec!['1']];
        assert_eq!(Solution::num_islands(case3), 1);
    }
}
