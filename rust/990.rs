/*

https://leetcode-cn.com/problems/satisfiability-of-equality-equations/

990. 等式方程的可满足性
给定一个由表示变量之间关系的字符串方程组成的数组，每个字符串方程 equations[i] 的长度为 4，并采用两种不同的形式之一："a==b" 或 "a!=b"。在这里，a 和 b 是小写字母（不一定不同），表示单字母变量名。

只有当可以将整数分配给变量名，以便满足所有给定的方程时才返回 true，否则返回 false。



示例 1：

输入：["a==b","b!=a"]
输出：false
解释：如果我们指定，a = 1 且 b = 1，那么可以满足第一个方程，但无法满足第二个方程。没有办法分配变量同时满足这两个方程。
示例 2：

输出：["b==a","a==b"]
输入：true
解释：我们可以指定 a = 1 且 b = 1 以满足满足这两个方程。
示例 3：

输入：["a==b","b==c","a==c"]
输出：true
示例 4：

输入：["a==b","b!=c","c==a"]
输出：false
示例 5：

输入：["c==c","b==d","x!=z"]
输出：true


提示：

1 <= equations.length <= 500
equations[i].length == 4
equations[i][0] 和 equations[i][3] 是小写字母
equations[i][1] 要么是 '='，要么是 '!'
equations[i][2] 是 '='
 */

struct Solution;

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let roots = &mut (0..26).collect::<Vec<_>>();
        fn get_parent(x: usize, roots: &mut Vec<usize>) -> usize {
            if roots[x] != x {
                let px = get_parent(roots[x], roots);
                roots[x] = px;
            }
            roots[x]
        };
        let to_int = |i| i as usize - 97;
        for s in equations.iter() {
            let s = s.as_bytes();
            if s[1] == s[2] {
                let (pi, pj) = (
                    get_parent(to_int(s[0]), roots),
                    get_parent(to_int(s[3]), roots),
                );
                if pi != pj {
                    roots[pj] = pi;
                }
            }
        }
        (0..26).for_each(|i| {
            get_parent(i, roots);
        });
        equations
            .into_iter()
            .map(|s| {
                let s = s.as_bytes();
                (s[1] == s[2])
                    == (get_parent(to_int(s[0]), roots) == get_parent(to_int(s[3]), roots))
            })
            .all(|b| b)
    }
}

fn main() {
    let input: Vec<String> = vec![
        "a==b".to_string(),
        "b==c".to_string(),
        "d==e".to_string(),
        "e==f".to_string(),
        "d==a".to_string(),
        "f!=a".to_string(),
    ];
    let results: bool = Solution::equations_possible(input);
    println!("{:?}", results);
}
