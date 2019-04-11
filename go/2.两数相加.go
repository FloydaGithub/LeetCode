/*
 * @lc app=leetcode.cn id=2 lang=golang
 *
 * [2] 两数相加
 *
 * https://leetcode-cn.com/problems/add-two-numbers/description/
 *
 * algorithms
 * Medium (33.76%)
 * Total Accepted:    112.4K
 * Total Submissions: 332.9K
 * Testcase Example:  '[2,4,3]\n[5,6,4]'
 *
 * 给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。
 *
 * 如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
 *
 * 您可以假设除了数字 0 之外，这两个数都不会以 0 开头。
 *
 * 示例：
 *
 * 输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
 * 输出：7 -> 0 -> 8
 * 原因：342 + 465 = 807
 *
 *
 */
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
package main

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	var v1, v2 int
	var _ret *ListNode = &ListNode{}
	var ret *ListNode = _ret
	for {
		if l1 != nil {
			v1 = l1.Val
			l1 = l1.Next
		} else {
			v1 = 0
		}
		if l2 != nil {
			v2 = l2.Val
			l2 = l2.Next
		} else {
			v2 = 0
		}
		ret.Val += v1 + v2
		if ret.Val >= 10 {
			ret.Next = &ListNode{Val: ret.Val / 10}
			ret.Val = ret.Val % 10
		}
		if l1 == nil && l2 == nil {
			break
		}
		if ret.Next == nil {
			ret.Next = &ListNode{}
		}
		ret = ret.Next
	}
	return _ret
}

/*
func main() {
	l1 := &ListNode{2, &ListNode{4, &ListNode{Val: 3}}}
	l2 := &ListNode{5, &ListNode{6, &ListNode{Val: 4}}}
	ret := addTwoNumbers(l1, l2)
	for {
		println("[2.两数相加.go] ret.Val", ret.Val)
		ret = ret.Next
		if ret == nil {
			break
		}
	}
}
*/

/*
√ Accepted
√ 1563/1563 cases passed (24 ms)
√ Your runtime beats 90.59 % of golang submissions
√ Your memory usage beats 36.92 % of golang submissions (5.1 MB)
*/
