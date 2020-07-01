/*

https://leetcode-cn.com/problems/kth-largest-element-in-an-array/

215. 数组中的第K个最大元素
在未排序的数组中找到第 k 个最大的元素。请注意，你需要找的是数组排序后的第 k 个最大的元素，而不是第 k 个不同的元素。

示例 1:

输入: [3,2,1,5,6,4] 和 k = 2
输出: 5
示例 2:

输入: [3,2,3,1,2,4,5,5,6] 和 k = 4
输出: 4
说明:

你可以假设 k 总是有效的，且 1 ≤ k ≤ 数组的长度。
 */

struct Solution;

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        nums.reverse();
        nums[k as usize - 1]
    }
}

fn main() {
    // let nums: Vec<i32> = vec![3, 2, 1, 5, 6, 4];
    // let k = 2;
    let nums: Vec<i32> = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
    let k = 4;
    let result: i32 = Solution::find_kth_largest(nums, k);
    println!("{:?}", result);
}
