/*

https://leetcode-cn.com/problems/longest-common-prefix/

14. 最长公共前缀
编写一个函数来查找字符串数组中的最长公共前缀。

如果不存在公共前缀，返回空字符串 ""。

示例 1:

输入: ["flower","flow","flight"]
输出: "fl"
示例 2:

输入: ["dog","racecar","car"]
输出: ""
解释: 输入不存在公共前缀。
说明:

所有输入只包含小写字母 a-z 。
 */

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return "".to_string();
        }
        let mut common: String = strs[0].to_string();
        fn compare_string(a: String, b: String) -> String {
            let mut index: usize = 0;
            let len_a = a.len();
            let len_b = b.len();
            loop {
                if index >= len_a || index >= len_b {
                    return a[0..index].to_string();
                }
                if a[index..index + 1] != b[index..index + 1] {
                    return a[0..index].to_string();
                }
                index += 1;
            }
        }

        for i in 1..strs.len() {
            common = compare_string(strs[i].to_string(), common);
            if common == "" {
                break;
            }
        }

        return common;
    }
}

fn main() {
    let input: Vec<String> = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    let result: String = Solution::longest_common_prefix(input);
    println!("{:?}", result);
}
