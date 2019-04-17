/*
 * @lc app=leetcode.cn id=14 lang=golang
 *
 * [14] 最长公共前缀
 *
 * https://leetcode-cn.com/problems/longest-common-prefix/description/
 *
 * algorithms
 * Easy (32.70%)
 * Total Accepted:    70.7K
 * Total Submissions: 215.5K
 * Testcase Example:  '["flower","flow","flight"]'
 *
 * 编写一个函数来查找字符串数组中的最长公共前缀。
 *
 * 如果不存在公共前缀，返回空字符串 ""。
 *
 * 示例 1:
 *
 * 输入: ["flower","flow","flight"]
 * 输出: "fl"
 *
 *
 * 示例 2:
 *
 * 输入: ["dog","racecar","car"]
 * 输出: ""
 * 解释: 输入不存在公共前缀。
 *
 *
 * 说明:
 *
 * 所有输入只包含小写字母 a-z 。
 *
 */

package main

func compareStr(a, b string) int {
	var index int = 0
	for {
		if index >= len(a) || index >= len(b) {
			return index
		}
		if a[index] != b[index] {
			return index
		}
		index++
	}
}

func longestCommonPrefix(strs []string) string {
	if len(strs) == 0 {
		return ""
	}
	var ret string = strs[0]
	var max int = len(ret)
	var l int
	for i := 1; i < len(strs); i++ {
		l = compareStr(ret, strs[i])
		if l == 0 {
			return ""
		}
		if l < max {
			max = l
		}
	}
	return ret[0:max]
}

func main() {
	var ret string
	ret = longestCommonPrefix([]string{"flower", "flow", "flight"})
	println("[14.最长公共前缀.go] ret", ret)
	ret = longestCommonPrefix([]string{"dog", "racecar", "car"})
	println("[14.最长公共前缀.go] ret", ret)
	ret = longestCommonPrefix([]string{"aca", "cba"})
	println("[14.最长公共前缀.go] ret", ret)
	ret = longestCommonPrefix([]string{"", ""})
	println("[14.最长公共前缀.go] ret", ret)
}

/*
√ Accepted
√ 118/118 cases passed (4 ms)
√ Your runtime beats 91.32 % of golang submissions
√ Your memory usage beats 48.92 % of golang submissions (2.4 MB)
*/
