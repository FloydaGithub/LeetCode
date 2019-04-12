/*
 * @lc app=leetcode.cn id=273 lang=golang
 *
 * [273] 整数转换英文表示
 *
 * https://leetcode-cn.com/problems/integer-to-english-words/description/
 *
 * algorithms
 * Hard (20.67%)
 * Total Accepted:    706
 * Total Submissions: 3.4K
 * Testcase Example:  '123'
 *
 * 将非负整数转换为其对应的英文表示。可以保证给定输入小于 2^31 - 1 。
 *
 * 示例 1:
 *
 * 输入: 123
 * 输出: "One Hundred Twenty Three"
 *
 *
 * 示例 2:
 *
 * 输入: 12345
 * 输出: "Twelve Thousand Three Hundred Forty Five"
 *
 * 示例 3:
 *
 * 输入: 1234567
 * 输出: "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
 *
 * 示例 4:
 *
 * 输入: 1234567891
 * 输出: "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven
 * Thousand Eight Hundred Ninety One"
 *
 */

package main

import (
	"strings"
)

var unit = [4]string{"", "Thousand", "Million", "Billion"}

var numToEnglish = map[int]string{
	0:   "Zero",
	1:   "One",
	2:   "Two",
	3:   "Three",
	4:   "Four",
	5:   "Five",
	6:   "Six",
	7:   "Seven",
	8:   "Eight",
	9:   "Nine",
	10:  "Ten",
	11:  "Eleven",
	12:  "Twelve",
	13:  "Thirteen",
	14:  "Fourteen",
	15:  "Fifteen",
	16:  "Sixteen",
	17:  "Seventeen",
	18:  "Eighteen",
	19:  "Nineteen",
	20:  "Twenty",
	30:  "Thirty",
	40:  "Forty",
	50:  "Fifty",
	60:  "Sixty",
	70:  "Seventy",
	80:  "Eighty",
	90:  "Ninety",
	100: "Hundred",
}

func num3ToWords(num int) string {
	if num == 0 {
		return ""
	}
	var ret []string
	a := num / 100
	b := num % 100 / 10
	c := num % 10

	if a != 0 {
		ret = append(ret, numToEnglish[a])
		ret = append(ret, "Hundred")
	}
	if b > 0 && b < 2 {
		ret = append(ret, numToEnglish[b*10+c])
	} else {
		if b != 0 {
			ret = append(ret, numToEnglish[b*10])
		}
		if c != 0 {
			ret = append(ret, numToEnglish[c])
		}
	}
	return strings.Join(ret, " ")
}

func numberToWords(num int) string {
	if num == 0 {
		return "Zero"
	}
	var ret, s3 string
	var n3 int
	var level int
	for {
		n3 = num % 1000
		num /= 1000

		s3 = num3ToWords(n3)
		tmp := []string{}
		if s3 != "" {
			tmp = append(tmp, s3)
			if level > 0 {
				tmp = append(tmp, unit[level])
			}
			tmp = append(tmp, ret)
			ret = strings.Join(tmp, " ")
		}
		level += 1
		if num == 0 {
			break
		}
	}
	if ret[len(ret)-1:len(ret)] == " " {
		return ret[0 : len(ret)-1]
	} else {
		return ret
	}
}

/*
func main() {
    var ret string
    ret = numberToWords(0) // Zero
    println("0:", ret)
    ret = numberToWords(10) // Ten
    println("10:", ret)
    ret = numberToWords(100) // One Hundred
    println("100:", ret)
    ret = numberToWords(1000) // One Thousand
    println("1000:", ret)
    ret = numberToWords(123) // One Hundred Twenty Three
    println("123:", ret)
    ret = numberToWords(12345) // Twelve Thousand Three Hundred Forty Five
    println("12345:", ret)
    ret = numberToWords(1234567) // One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven
    println("1234567:", ret)
    ret = numberToWords(1234567891) // One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One
    println("1234567891:", ret)
}
*/
