/*

https://leetcode-cn.com/problems/minimum-size-subarray-sum/

209. 长度最小的子数组
给定一个含有 n 个正整数的数组和一个正整数 s ，找出该数组中满足其和 ≥ s 的长度最小的连续子数组，并返回其长度。如果不存在符合条件的连续子数组，返回 0。

示例:

输入: s = 7, nums = [2,3,1,2,4,3]
输出: 2
解释: 子数组 [4,3] 是该条件下的长度最小的连续子数组。
进阶:

如果你已经完成了O(n) 时间复杂度的解法, 请尝试 O(n log n) 时间复杂度的解法。

通过次数57,976提交次数132,870

 */

struct Solution;

impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut left = 0;
        let mut right = 0;
        let mut total = 0;
        let mut min = i32::max_value();

        while left < len {
            total += nums[right];
            right += 1;
            if right >= len || total >= s {
                if total >= s {
                    min = std::cmp::min(min, (right - left) as i32);
                }
                left += 1;
                right = left;
                total = 0;
            }
            if left >= len {
                break;
            }
        }
        if min == i32::max_value() {
            return 0;
        }
        min
    }
}

fn main() {
    let s = 7;
    let nums: Vec<i32> = vec![2, 3, 1, 2, 4, 3];
    // let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    let result = Solution::min_sub_array_len(s, nums);
    println!("{:?}", result);
}
