struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for ch in s.chars() {
            match ch {
                '(' | '[' | '{' => stack.push(ch),
                ')' => {
                    if stack.pop() != Some('('){return false;}
                },
                ']' => {
                    if stack.pop() != Some('['){return false;}
                },
                '}' => {
                    if stack.pop() != Some('{'){return false;}
                },
                _ => return false,
            }
        }
        stack.is_empty()
    }
}

//test
fn main(){
let test: String = String::from("()[]");
let result = Solution::is_valid(test);
println!("{}", result);
}

