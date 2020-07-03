/*

https://leetcode-cn.com/problems/convert-sorted-array-to-binary-search-tree/

108. 将有序数组转换为二叉搜索树
将一个按照升序排列的有序数组，转换为一棵高度平衡二叉搜索树。

本题中，一个高度平衡二叉树是指一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过 1。

示例:

给定有序数组: [-10,-3,0,5,9],

一个可能的答案是：[0,-3,9,-10,null,5]，它可以表示下面这个高度平衡二叉搜索树：

      0
     / \
   -3   9
   /   /
 -10  5

 */

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if !nums.is_empty() {
            Some(Rc::new(RefCell::new(TreeNode {
                val: nums[nums.len() >> 1],
                left: Solution::sorted_array_to_bst(nums[..nums.len() >> 1].to_vec()),
                right: Solution::sorted_array_to_bst(nums[(nums.len() >> 1) + 1..].to_vec()),
            })))
        } else {
            None
        }
    }
}

fn main() {
    let nums: Vec<i32> = vec![-10, -3, 0, 5, 9];
    let result = Solution::sorted_array_to_bst(nums);
    println!("{:#?}", result);
}
