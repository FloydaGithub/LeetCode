/*
 * @lc app=leetcode.cn id=7 lang=golang
 *
 * [7] 整数反转
 *
 * https://leetcode-cn.com/problems/reverse-integer/description/
 *
 * algorithms
 * Easy (32.22%)
 * Total Accepted:    109.7K
 * Total Submissions: 340.6K
 * Testcase Example:  '123'
 *
 * 给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。
 *
 * 示例 1:
 *
 * 输入: 123
 * 输出: 321
 *
 *
 * 示例 2:
 *
 * 输入: -123
 * 输出: -321
 *
 *
 * 示例 3:
 *
 * 输入: 120
 * 输出: 21
 *
 *
 * 注意:
 *
 * 假设我们的环境只能存储得下 32 位的有符号整数，则其数值范围为 [−2^31,  2^31 − 1]。请根据这个假设，如果反转后整数溢出那么就返回
 * 0。
 *
 */
package main

func reverse(x int) int {
	var max int = 2<<30 - 1
	var min int = -2 << 30
	var ret int = 0
	for {
		ret = ret*10 + x%10
		x = x / 10
		if ret > max || ret < min {
			return 0
		}
		if x == 0 {
			return ret
		}
	}
	return 0
}

/*
func main() {
	var ret int
	ret = reverse(123)
	println("[7.整数反转.go] ret", ret)
	ret = reverse(-123)
	println("[7.整数反转.go] ret", ret)
	ret = reverse(120)
	println("[7.整数反转.go] ret", ret)
}
*/

/*
√ Accepted
√ 1032/1032 cases passed (8 ms)
√ Your runtime beats 91.3 % of golang submissions
√ Your memory usage beats 55.1 % of golang submissions (2.2 MB)
*/
