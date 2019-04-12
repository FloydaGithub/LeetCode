/*
 * @lc app=leetcode.cn id=13 lang=golang
 *
 * [13] 罗马数字转整数
 *
 * https://leetcode-cn.com/problems/roman-to-integer/description/
 *
 * algorithms
 * Easy (57.31%)
 * Total Accepted:    53.3K
 * Total Submissions: 93K
 * Testcase Example:  '"III"'
 *
 * 罗马数字包含以下七种字符: I， V， X， L，C，D 和 M。
 *
 * 字符          数值
 * I             1
 * V             5
 * X             10
 * L             50
 * C             100
 * D             500
 * M             1000
 *
 * 例如， 罗马数字 2 写做 II ，即为两个并列的 1。12 写做 XII ，即为 X + II 。 27 写做  XXVII, 即为 XX + V +
 * II 。
 *
 * 通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 IIII，而是 IV。数字 1 在数字 5 的左边，所表示的数等于大数
 * 5 减小数 1 得到的数值 4 。同样地，数字 9 表示为 IX。这个特殊的规则只适用于以下六种情况：
 *
 *
 * I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
 * X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。
 * C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。
 *
 *
 * 给定一个罗马数字，将其转换成整数。输入确保在 1 到 3999 的范围内。
 *
 * 示例 1:
 *
 * 输入: "III"
 * 输出: 3
 *
 * 示例 2:
 *
 * 输入: "IV"
 * 输出: 4
 *
 * 示例 3:
 *
 * 输入: "IX"
 * 输出: 9
 *
 * 示例 4:
 *
 * 输入: "LVIII"
 * 输出: 58
 * 解释: L = 50, V= 5, III = 3.
 *
 *
 * 示例 5:
 *
 * 输入: "MCMXCIV"
 * 输出: 1994
 * 解释: M = 1000, CM = 900, XC = 90, IV = 4.
 *
 */
package main

var charmap = map[string]int{
	"I": 1,
	"V": 5,
	"X": 10,
	"L": 50,
	"C": 100,
	"D": 500,
	"M": 1000,
}

func romanToInt(s string) int {
	var length int = len(s)
	var ret int = 0
	var s_last, s_current string
	var v_last, v_current int

	s_last = s[length-1 : length]
	v_last = charmap[s_last]
	ret = v_last

	for i := length - 1; i > 0; i-- {
		s_current = s[i-1 : i]
		v_current = charmap[s_current]
		if v_last > v_current {
			ret -= v_current
		} else {
			ret += v_current
		}
		s_last = s_current
		v_last = v_current
	}
	return ret
}

/*
func main() {
    var ret int
    ret = romanToInt("III")
    println("[13.罗马数字转整数.go] ret", ret)
    ret = romanToInt("IV")
    println("[13.罗马数字转整数.go] ret", ret)
    ret = romanToInt("IX")
    println("[13.罗马数字转整数.go] ret", ret)
    ret = romanToInt("LVIII")
    println("[13.罗马数字转整数.go] ret", ret)
    ret = romanToInt("MCMXCIV")
    println("[13.罗马数字转整数.go] ret", ret)
}
*/

/*
√ Accepted
√ 3999/3999 cases passed (24 ms)
√ Your runtime beats 97.3 % of golang submissions
√ Your memory usage beats 20.48 % of golang submissions (3.2 MB)
*/
