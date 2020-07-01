/*

https://leetcode-cn.com/problems/maximum-length-of-repeated-subarray/

718. 最长重复子数组
给两个整数数组 A 和 B ，返回两个数组中公共的、长度最长的子数组的长度。

示例 1:

输入:
A: [1,2,3,2,1]
B: [3,2,1,4,7]
输出: 3
解释:
长度最长的公共子数组是 [3, 2, 1]。
说明:

1 <= len(A), len(B) <= 1000
0 <= A[i], B[i] < 100

 */

struct Solution;

impl Solution {
    pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let (m, n) = (a.len(), b.len());
        // c[i][0] 表示 a[i] 与空字符串的最长公共子数组的长度。
        let mut c = vec![vec![0; n + 1]; m + 1];
        let mut ans = 0;
        for i in 1..=m {
            for j in 1..=n {
                if a[i - 1] == b[j - 1] {
                    // 相等的话，a[i] 是公共子数组的一部分
                    c[i][j] = c[i - 1][j - 1] + 1;
                    ans = ans.max(c[i][j]);
                } else {
                    // 不相等的话，就不连续了，设置为 0
                    c[i][j] = 0;
                }
            }
        }

        ans
    }
}

fn main() {
    let a: Vec<i32> = vec![1, 2, 3, 2, 1];
    let b: Vec<i32> = vec![3, 2, 1, 4, 7];
    let result = Solution::find_length(a, b);
    println!("{:?}", result);
}
