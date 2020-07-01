/*

https://leetcode-cn.com/problems/yong-liang-ge-zhan-shi-xian-dui-lie-lcof/

剑指 Offer 09. 用两个栈实现队列
用两个栈实现一个队列。队列的声明如下，请实现它的两个函数 appendTail 和 deleteHead ，分别完成在队列尾部插入整数和在队列头部删除整数的功能。(若队列中没有元素，deleteHead 操作返回 -1 )



示例 1：

输入：
["CQueue","appendTail","deleteHead","deleteHead"]
[[],[3],[],[]]
输出：[null,null,3,-1]

示例 2：

输入：
["CQueue","deleteHead","appendTail","appendTail","deleteHead","deleteHead"]
[[],[],[5],[2],[],[]]
输出：[null,-1,null,null,5,2]
提示：

1 <= values <= 10000
最多会对 appendTail、deleteHead 进行 10000 次调用

 */

#[derive(Default, Debug)]
struct CQueue {
    p: Vec<i32>,
    q: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {
    fn new() -> Self {
        Default::default()
    }

    fn append_tail(&mut self, value: i32) {
        self.p.push(value);
    }

    fn delete_head(&mut self) -> i32 {
        match self.q.pop() {
            Some(val) => val,
            None => {
                self.p.reverse();
                self.q.append(&mut self.p);
                self.q.pop().unwrap_or(-1)
            }
        }
    }
}

/**
 * Your CQueue object will be instantiated and called as such:
 * let obj = CQueue::new();
 * obj.append_tail(value);
 * let ret_2: i32 = obj.delete_head();
 */

fn main() {
    let obj = CQueue::new();
    // obj.append_tail(value);
    let ret_2: i32 = obj.delete_head();
    println!("{}", obj.p);
}
