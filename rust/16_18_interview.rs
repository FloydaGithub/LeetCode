/*

https://leetcode-cn.com/problems/pattern-matching-lcci/

面试题 16.18. 模式匹配

你有两个字符串，即pattern和value。 pattern字符串由字母"a"和"b"组成，用于描述字符串中的模式。例如，字符串"catcatgocatgo"匹配模式"aabab"（其中"cat"是"a"，"go"是"b"），该字符串也匹配像"a"、"ab"和"b"这样的模式。但需注意"a"和"b"不能同时表示相同的字符串。编写一个方法判断value字符串是否匹配pattern字符串。

示例 1：

输入： pattern = "abba", value = "dogcatcatdog"
输出： true
示例 2：

输入： pattern = "abba", value = "dogcatcatfish"
输出： false
示例 3：

输入： pattern = "aaaa", value = "dogcatcatdog"
输出： false
示例 4：

输入： pattern = "abba", value = "dogdogdogdog"
输出： true
解释： "a"="dogdog",b=""，反之也符合规则
提示：

0 <= len(pattern) <= 1000
0 <= len(value) <= 1000
你可以假设pattern只包含字母"a"和"b"，value仅包含小写字母。*/

struct Solution;

impl Solution {
    pub fn pattern_matching(pattern: String, value: String) -> bool {
        let len_pattern = pattern.len();
        let len_value = value.len();
        // 判断边界
        if len_value == 0 {
            match len_pattern {
                0 => return true,
                1 => return true,
                _ => return false,
            }
        }

        let c: Vec<char> = pattern.chars().collect();

        for a in 0..len_value + 1 {
            for b in 0..len_value + 1 {
                let mut flag = true;
                let mut a_result = "...";
                let mut b_result = "...";
                let mut a_total = 0;
                let mut b_total = 0;

                for i in 0..len_pattern {
                    let _p = c[i];
                    if _p == 'a' {
                        let index = a * a_total + b * b_total;
                        if index + a > len_value {
                            flag = false;
                            break;
                        }
                        let common = &value[index..index + a];
                        if a_result == "..." {
                            a_result = common;
                        } else if common != a_result {
                            flag = false;
                            break;
                        }
                        a_total += 1;
                    }
                    if _p == 'b' {
                        let index = a * a_total + b * b_total;
                        if index + b > len_value {
                            flag = false;
                            break;
                        }
                        let common = &value[index..index + b];
                        if b_result == "..." {
                            b_result = common;
                        } else if common != b_result {
                            flag = false;
                            break;
                        }
                        b_total += 1;
                    }
                }
                // 没有不匹配 & 字符串长度要对应 & "a"和"b"不能同时表示相同的字符串
                if flag && a * a_total + b * b_total == len_value && a_result != b_result {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    // let (pattern, value) = ("a", ""); // true
    // let (pattern, value) = ("ab", ""); // false
    // let (pattern, value) = ("abba", "dogcatcatfish"); // false
    // let (pattern, value) = ("aaaa", "dogcatcatdog"); // false
    // let (pattern, value) = ("abba", "dogcatcatdog"); // true
    // let (pattern, value) = ("abba", "dogdogdogdog"); // true
    let (pattern, value) = ("aaaaab", "xahnxdxyaahnxdxyaahnxdxyaahnxdxyaauxuhuo"); // true
    let pattern = pattern.to_string();
    let value = value.to_string();
    let result: bool = Solution::pattern_matching(pattern, value);
    println!("{:?}", result);
}
