// 示例：

// 输入：A = [4,5,0,-2,-3,1], K = 5
// 输出：7
// 解释：
// 有 7 个子数组满足其元素之和可被 K = 5 整除：
// [4, 5, 0, -2, -3, 1], [5], [5, 0], [5, 0, -2, -3], [0], [0, -2, -3], [-2, -3]
//

// 提示：

// 1 <= A.length <= 30000
// -10000 <= A[i] <= 10000
// 2 <= K <= 10000

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/subarray-sums-divisible-by-k
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

struct Solution;

impl Solution {
    pub fn subarrays_div_by_k(a: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut s = 0;
        let mut t = vec![0; k as usize];
        t[0] = 1;
        for i in a.into_iter() {
            s = ((s + i) % k + k) % k;
            ans += t[s as usize];
            t[s as usize] += 1;
        }
        ans
    }
}

fn main() {
    let result = Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5);
    println!("{:?}", result);
}
