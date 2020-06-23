/*

https://leetcode-cn.com/problems/add-binary/

67. 二进制求和
给你两个二进制字符串，返回它们的和（用二进制表示）。

输入为 非空 字符串且只包含数字 1 和 0。



示例 1:

输入: a = "11", b = "1"
输出: "100"
示例 2:

输入: a = "1010", b = "1011"
输出: "10101"


提示：

每个字符串仅由字符 '0' 或 '1' 组成。
1 <= a.length, b.length <= 10^4
字符串如果不是 "0" ，就都不含前导零。
 */

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut index_a = a.len() as i32;
        let mut index_b = b.len() as i32;
        let mut increase = 0;
        let mut result: String = String::new();

        let char_a: Vec<char> = a.chars().collect();
        let char_b: Vec<char> = b.chars().collect();

        loop {
            index_a -= 1;
            index_b -= 1;

            let mut aa = 0;
            let mut bb = 0;
            if index_a >= 0 {
                aa = char_a[index_a as usize].to_digit(10).unwrap_or(0);
            }
            if index_b >= 0 {
                bb = char_b[index_b as usize].to_digit(10).unwrap_or(0);
            }

            let total = aa + bb + increase;
            increase = total / 2;
            result = (total % 2).to_string() + &result;

            if index_a <= 0 && index_b <= 0 {
                break;
            }
        }

        if increase > 0 {
            result = "1".to_string() + &result;
        }

        result
    }
}

fn main() {
    // let a = "11".to_string();
    // let b = "1".to_string();
    let a = "100".to_string();
    let b = "100".to_string();
    let result: String = Solution::add_binary(a, b);
    println!("{:?}", result);
}
