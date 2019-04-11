/*
 * @lc app=leetcode.cn id=1 lang=golang
 *
 * [1] 两数之和
 *
 * https://leetcode-cn.com/problems/two-sum/description/
 *
 * algorithms
 * Easy (45.70%)
 * Total Accepted:    326.2K
 * Total Submissions: 713.9K
 * Testcase Example:  '[2,7,11,15]\n9'
 *
 * 给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
 *
 * 你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。
 *
 * 示例:
 *
 * 给定 nums = [2, 7, 11, 15], target = 9
 *
 * 因为 nums[0] + nums[1] = 2 + 7 = 9
 * 所以返回 [0, 1]
 *
 *
 */
package main

func twoSum(nums []int, target int) []int {
	var ret []int
	var hash map[int]int = make(map[int]int)

	for i := 0; i < len(nums); i++ {
		num := nums[i]
		pos, ok := hash[num]
		if ok {
			return []int{pos, i}
		}
		hash[target-num] = i
	}

	return ret
}

// func main() {
// 	var nums []int = []int{2, 7, 11, 15}
// 	a := twoSum(nums, 9)
// 	fmt.Println(a)
// }

/*
√ Accepted
√ 29/29 cases passed (8 ms)
√ Your runtime beats 96.66 % of golang submissions
√ Your memory usage beats 21.11 % of golang submissions (3.8 MB)
*/
