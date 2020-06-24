/*

https://leetcode-cn.com/problems/3sum-closest/

16. 最接近的三数之和
给定一个包括 n 个整数的数组 nums 和 一个目标值 target。找出 nums 中的三个整数，使得它们的和与 target 最接近。返回这三个数的和。假定每组输入只存在唯一答案。



示例：

输入：nums = [-1,2,1,-4], target = 1
输出：2
解释：与 target 最接近的和是 2 (-1 + 2 + 1 = 2) 。


提示：

3 <= nums.length <= 10^3
-10^3 <= nums[i] <= 10^3
-10^4 <= target <= 10^4
 */

struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let len_nums = nums.len();
        let mut result: i32 = nums[0] + nums[1] + nums[2];

        for i in 0..len_nums - 2 {
            let mut start = i + 1;
            let mut end = len_nums - 1;
            while start < end {
                let sum = nums[start] + nums[end] + nums[i];
                if (target - sum).abs() < (target - result).abs() {
                    result = sum;
                }
                if sum > target {
                    end -= 1;
                } else if sum < target {
                    start += 1;
                } else {
                    return result;
                }
            }
        }
        result
    }
}

fn main() {
    let nums: Vec<i32> = vec![-1, 2, 1, -4];
    let target: i32 = 1;
    let result: i32 = Solution::three_sum_closest(nums, target);
    println!("{:?}", result);
}
