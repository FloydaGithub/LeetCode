/*

https://leetcode-cn.com/problems/diving-board-lcci/

面试题 16.11. 跳水板
你正在使用一堆木板建造跳水板。有两种类型的木板，其中长度较短的木板长度为shorter，长度较长的木板长度为longer。你必须正好使用k块木板。编写一个方法，生成跳水板所有可能的长度。

返回的长度需要从小到大排列。

示例：

输入：
shorter = 1
longer = 2
k = 3
输出： {3,4,5,6}
提示：

0 < shorter <= longer
0 <= k <= 100000

 */

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn diving_board(shorter: i32, longer: i32, k: i32) -> Vec<i32> {
        let mut hash = HashMap::new();
        for longer_num in 0..k + 1 {
            let shorter_num = k - longer_num;
            let total = shorter_num * shorter + longer_num * longer;
            hash.insert(total, true);
        }

        let mut ans: Vec<i32> = Vec::new();
        for (k, _) in hash.iter() {
            if *k != 0 {
                ans.push(*k);
            }
        }
        ans.sort();
        ans
    }
}

fn main() {
    let shorter: i32 = 1;
    let longer: i32 = 2;
    let k: i32 = 3;
    let result: Vec<i32> = Solution::diving_board(shorter, longer, k);
    println!("{:?}", result);
}
