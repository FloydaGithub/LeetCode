/*

https://leetcode-cn.com/problems/unique-paths-ii/

63. 不同路径 II
一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为“Start” ）。

机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为“Finish”）。

现在考虑网格中有障碍物。那么从左上角到右下角将会有多少条不同的路径？



网格中的障碍物和空位置分别用 1 和 0 来表示。

说明：m 和 n 的值均不超过 100。

示例 1:

输入:
[
  [0,0,0],
  [0,1,0],
  [0,0,0]
]
输出: 2
解释:
3x3 网格的正中间有一个障碍物。
从左上角到右下角一共有 2 条不同的路径：
1. 向右 -> 向右 -> 向下 -> 向下
2. 向下 -> 向下 -> 向右 -> 向右

 */

#[warn(unused_mut)]
struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();

        let mut cache = vec![vec![0; n]; m]; // 创建一个 m x n 的路径数矩阵

        for i in 0..m {
            for j in 0..n {
                if obstacle_grid[i][j] == 1 {
                    // 如果这一点有障碍物，则路径数为0
                    cache[i][j] = 0;
                } else if i == 0 && j == 0 {
                    // 如果是初始值，则路径数为1
                    cache[i][j] = 1;
                } else if i == 0 {
                    cache[i][j] = cache[i][j - 1]; // 见上面的解释
                } else if j == 0 {
                    cache[i][j] = cache[i - 1][j]; // 见上面的解释
                } else {
                    cache[i][j] = cache[i - 1][j] + cache[i][j - 1];
                }
            }
        }
        return cache[m - 1][n - 1];
    }
}

fn main() {
    let obstacle_grid: Vec<Vec<i32>> = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    let result = Solution::unique_paths_with_obstacles(obstacle_grid);
    println!("{:?}", result);
}
