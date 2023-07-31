struct Solution;

struct Group {
    s: String,
    n: i32,
    u: i32
}

use std::collections::VecDeque;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let g: usize = n as usize * 2;
        let mut queue: VecDeque<Group> = VecDeque::new();
        if n == 0 {
            return vec!["".to_string()];
        }
        queue.push_front(Group {s: "(".to_string(), n: n - 1, u: 1});
        while queue.front().unwrap().s.len() < g {
            let cur: Group = queue.pop_front().unwrap();
            if cur.n > 0 {
                queue.push_back(Group { s: (cur.s.clone() + "("), n: (cur.n.clone() - 1), u: (cur.u.clone() + 1) });
            }
            if cur.u > 0 {
                queue.push_back(Group { s: (cur.s.clone() + ")"), n: (cur.n.clone()), u: (cur.u.clone() - 1) });
            }
        }
        let mut res: Vec<String> = Vec::new();
        while queue.len() > 0 {
            res.push(queue.pop_front().unwrap().s);
        }
        return res;
    }
}

fn main() {
    let i: i32 = 3;
    let result: Vec<String> = Solution::generate_parenthesis(i);
    println!("{:?}", result);
}
