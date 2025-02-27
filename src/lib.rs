use std::collections::HashMap;

pub struct Solution;

/// Solves the problem of selecting cells in a grid to maximize the score.
///
/// Given a 2D grid of integers, the function calculates the maximum score
/// achievable by moving only right or down from the top-left corner to the
/// bottom-right corner.
///
/// # Arguments
///
/// * `grid` - A 2D vector of integers representing the grid.
///
/// # Returns
///
/// The maximum score as an `i32`.
impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        // Flatten the grid into a vector of (value, row, col)
        let mut values: Vec<(i32, usize, usize)> = Vec::new();
        for i in 0..n {
            for j in 0..m {
                values.push((grid[i][j], i, j));
            }
        }

        // Sort values in descending order
        values.sort_by(|a, b| b.0.cmp(&a.0));

        // Initialize DP cache
        let mut dp: HashMap<(usize, usize), i32> = HashMap::new();

        // Call the recursive function
        Self::recur(&values, 0, 0, &mut dp)
    }

    fn recur(
        values: &[(i32, usize, usize)],
        idx: usize,
        mask_row: usize,
        dp: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        let n = values.len();

        // Base case: if we've processed all cells
        if idx == n {
            return 0;
        }

        // Check if the result is already in the DP cache
        if let Some(&result) = dp.get(&(idx, mask_row)) {
            return result;
        }

        let (value, row, _) = values[idx];
        let mut ans = 0;

        // Check if the row is already selected
        if (1 << row) & mask_row != 0 {
            // Row is already selected, skip this cell
            ans = Self::recur(values, idx + 1, mask_row, dp);
        } else {
            // Row is not selected, decide whether to select this cell or not
            let mut j = idx;
            while j < n && values[j].0 == values[idx].0 {
                j += 1;
            }

            // Option 1: Select this cell and mark the row as selected
            let ans1 = value + Self::recur(values, j, mask_row | (1 << row), dp);

            // Option 2: Skip this cell
            let ans2 = Self::recur(values, idx + 1, mask_row, dp);

            // Choose the maximum of the two options
            ans = ans1.max(ans2);
        }

        // Store the result in the DP cache
        dp.insert((idx, mask_row), ans);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_score() {
        // Test case 1
        let grid1 = vec![
            vec![1, 2, 3],
            vec![4, 3, 2],
            vec![1, 1, 1],
        ];
        assert_eq!(Solution::max_score(grid1), 8);

        // Test case 2
        let grid2 = vec![
            vec![8, 7, 6],
            vec![8, 3, 2],
        ];
        assert_eq!(Solution::max_score(grid2), 15);
    }
}