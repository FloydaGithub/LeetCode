/*
求 1+2+...+n ，要求不能使用乘除法、for、while、if、else、switch、case等关键字及条件判断语句（A?B:C）。



示例 1：

输入: n = 3
输出: 6
示例 2：

输入: n = 9
输出: 45


限制：

1 <= n <= 10000
 */

struct Solution;

impl Solution {
    pub fn sum_nums(n: i32) -> i32 {
        (1..n + 1).fold(0, |i, j| i + j)
    }
}

fn main() {
    let result: i32 = Solution::sum_nums(2);
    println!("{:?}", result);
}
