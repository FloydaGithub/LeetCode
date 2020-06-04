/*
238. 除自身以外数组的乘积
给你一个长度为 n 的整数数组 nums，其中 n > 1，返回输出数组 output ，其中 output[i] 等于 nums 中除 nums[i] 之外其余各元素的乘积。



示例:

输入: [1,2,3,4]
输出: [24,12,8,6]


提示：题目数据保证数组之中任意元素的全部前缀元素和后缀（甚至是整个数组）的乘积都在 32 位整数范围内。

说明: 请不要使用除法，且在 O(n) 时间复杂度内完成此题。

进阶：
你可以在常数空间复杂度内完成这个题目吗？（ 出于对空间复杂度分析的目的，输出数组不被视为额外空间。）
 */

struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let num_length = nums.len();
        let mut front_product = vec![1; num_length];
        let mut back_product = vec![1; num_length];
        let mut output = vec![1; num_length];

        front_product[0] = nums[0];
        for i in 1..num_length {
            front_product[i] = front_product[i - 1] * nums[i];
        }

        back_product[num_length - 1] = nums[num_length - 1];
        for i in (0..num_length - 1).rev() {
            back_product[i] = back_product[i + 1] * nums[i];
        }

        output[0] = back_product[1];
        output[num_length - 1] = front_product[num_length - 2];
        for i in 1..num_length - 1 {
            output[i] = front_product[i - 1] * back_product[i + 1];
        }

        output
    }
}

fn main() {
    let result = Solution::product_except_self(vec![1, 2, 3, 4]);
    println!("{:?}", result);
}
