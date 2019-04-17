/*
 * @lc app=leetcode.cn id=3 lang=golang
 *
 * [3] 无重复字符的最长子串
 *
 * https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/description/
 *
 * algorithms
 * Medium (29.02%)
 * Total Accepted:    109K
 * Total Submissions: 375.3K
 * Testcase Example:  '"abcabcbb"'
 *
 * 给定一个字符串，请你找出其中不含有重复字符的 最长子串 的长度。
 *
 * 示例 1:
 *
 * 输入: "abcabcbb"
 * 输出: 3
 * 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
 *
 *
 * 示例 2:
 *
 * 输入: "bbbbb"
 * 输出: 1
 * 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
 *
 *
 * 示例 3:
 *
 * 输入: "pwwkew"
 * 输出: 3
 * 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
 * 请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
 *
 *
 */

package main

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func lengthOfLongestSubstring(s string) int {
	var ret int
	var hash = make(map[byte]int)
	i := 0
	for j := 0; j < len(s); j++ {
		i = max(hash[s[j]], i)
		ret = max(ret, j-i+1)
		hash[s[j]] = j + 1
	}
	return ret
}

/*func main() {
	var ret int
	ret = lengthOfLongestSubstring(" ")
	println("[3.无重复字符的最长子串.go] ret", ret)
	ret = lengthOfLongestSubstring("abcabcbb")
	println("[3.无重复字符的最长子串.go] ret", ret)
	ret = lengthOfLongestSubstring("bbbbb")
	println("[3.无重复字符的最长子串.go] ret", ret)
	ret = lengthOfLongestSubstring("pwwkew")
	println("[3.无重复字符的最长子串.go] ret", ret)
}
*/
