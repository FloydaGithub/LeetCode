/*

https://leetcode-cn.com/problems/recover-a-tree-from-preorder-traversal/

1028. 从先序遍历还原二叉树
我们从二叉树的根节点 root 开始进行深度优先搜索。

在遍历中的每个节点处，我们输出 D 条短划线（其中 D 是该节点的深度），然后输出该节点的值。（如果节点的深度为 D，则其直接子节点的深度为 D + 1。根节点的深度为 0）。

如果节点只有一个子节点，那么保证该子节点为左子节点。

给出遍历输出 S，还原树并返回其根节点 root。



示例 1：



输入："1-2--3--4-5--6--7"
输出：[1,2,5,3,4,6,7]
示例 2：



输入："1-2--3---4-5--6---7"
输出：[1,2,5,3,null,6,null,4,null,7]
示例 3：



输入："1-401--349---90--88"
输出：[1,401,null,349,88,90]


提示：

原始树中的节点数介于 1 和 1000 之间。
每个节点的值介于 1 和 10 ^ 9 之间。
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

struct Solution;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ans: HashMap<i32, Option<Rc<RefCell<TreeNode>>>> = HashMap::new();
        ans.insert(-1, Some(Rc::new(RefCell::new(TreeNode::new(0)))));
        let mut add_tree = |v, p| {
            ans.insert(p, Some(Rc::new(RefCell::new(TreeNode::new(v)))));
            let mut r = ans[&(p - 1)].as_ref().unwrap().borrow_mut();
            if r.left.is_none() {
                r.left = ans[&p].clone();
            } else {
                r.right = ans[&p].clone();
            }
        };
        let mut val = 0;
        let mut dep = 0;
        let mut has_val = false;
        for c in s.chars() {
            if c != '-' {
                val = 10 * val + c as i32 - 48;
                has_val = true;
            } else if has_val {
                add_tree(val, dep);
                val = 0;
                dep = 1;
                has_val = false;
            } else {
                dep += 1;
            }
        }
        add_tree(val, dep);
        ans[&0].clone()
    }
}

fn main() {
    let input: String = "1-2--3--4-5--6--7".to_string();
    let result = Solution::recover_from_preorder(input);
    println!("{:?}", result);
}
