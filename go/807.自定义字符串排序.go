/*
 * @lc app=leetcode.cn id=807 lang=golang
 *
 * [807] 自定义字符串排序
 *
 * https://leetcode-cn.com/problems/max-increase-to-keep-city-skyline/description/
 *
 * algorithms
 * Medium (74.99%)
 * Total Accepted:    3.2K
 * Total Submissions: 4.2K
 * Testcase Example:  '[[3,0,8,4],[2,4,5,7],[9,2,6,3],[0,3,1,0]]'
 *
 * 在二维数组grid中，grid[i][j]代表位于某处的建筑物的高度。 我们被允许增加任何数量（不同建筑物的数量可能不同）的建筑物的高度。 高度 0
 * 也被认为是建筑物。
 *
 * 最后，从新数组的所有四个方向（即顶部，底部，左侧和右侧）观看的“天际线”必须与原始数组的天际线相同。
 * 城市的天际线是从远处观看时，由所有建筑物形成的矩形的外部轮廓。 请看下面的例子。
 *
 * 建筑物高度可以增加的最大总和是多少？
 *
 *
 * 例子：
 * 输入： grid = [[3,0,8,4],[2,4,5,7],[9,2,6,3],[0,3,1,0]]
 * 输出： 35
 * 解释：
 * The grid is:
 * [ [3, 0, 8, 4],
 * ⁠ [2, 4, 5, 7],
 * ⁠ [9, 2, 6, 3],
 * ⁠ [0, 3, 1, 0] ]
 *
 * 从数组竖直方向（即顶部，底部）看“天际线”是：[9, 4, 8, 7]
 * 从水平水平方向（即左侧，右侧）看“天际线”是：[8, 7, 9, 3]
 *
 * 在不影响天际线的情况下对建筑物进行增高后，新数组如下：
 *
 * gridNew = [ [8, 4, 8, 7],
 * ⁠           [7, 4, 7, 7],
 * ⁠           [9, 4, 8, 7],
 * ⁠           [3, 3, 3, 3] ]
 *
 *
 * 说明:
 *
 *
 * 1 < grid.length = grid[0].length <= 50。
 * grid[i][j] 的高度范围是： [0, 100]。
 * 一座建筑物占据一个grid[i][j]：换言之，它们是 1 x 1 x grid[i][j] 的长方体。
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

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func maxIncreaseKeepingSkyline(grid [][]int) int {
	var ret int = 0
	var length = len(grid[0])
	var skyline_h []int
	var skyline_v []int

	for i := 0; i < length; i++ {
		var max_h int
		var max_v int
		for j := 0; j < length; j++ {
			max_h = max(grid[i][j], max_h)
			max_v = max(grid[j][i], max_v)
		}
		skyline_h = append(skyline_h, max_h)
		skyline_v = append(skyline_v, max_v)
	}

	for i := 0; i < length; i++ {
		for j := 0; j < length; j++ {
			ret += min(skyline_h[i]-grid[i][j], skyline_v[j]-grid[i][j])
		}
	}

	return ret
}

/*
func main() {
    grid := [][]int{{3, 0, 8, 4}, {2, 4, 5, 7}, {9, 2, 6, 3}, {0, 3, 1, 0}}
    ret := maxIncreaseKeepingSkyline(grid)
    println("[807.go] ret", ret)
}
*/

/*
√ Accepted
√ 133/133 cases passed (12 ms)
√ Your runtime beats 94.74 % of golang submissions
√ Your memory usage beats 35.71 % of golang submissions (3.5 MB)
*/

/*
Note:
首先找到天际线, 遍历二维数组, 找到每行每列的最大值.
然后再遍历一次二维数组, 根据横竖的天际线(最大值), 找到其中的`短板`, 即为该建筑物可增加的数量.
*/
