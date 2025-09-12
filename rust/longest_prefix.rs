struct Solution;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }       

        let min_len = strs.iter().map(|a| a.len()).min().unwrap();
        let mut prefix = String::new();

        for i in 0..min_len {
            let c = strs[0].as_bytes()[i];

            if strs.iter().any(|s| s.as_bytes()[i] != c) {
                break;
            }
            prefix.push(c as char);
        }
        prefix
    }
}


fn main() {
    let test:Vec<String> = ["lisa", "lily", "libra"].into_iter().map(String::from).collect();
    let result = Solution::longest_common_prefix(test);
    println!("{}", result)
}
