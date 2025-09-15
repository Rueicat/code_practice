//文字反轉
//但遇到uüu 會有問題, 要使用字叢的方式

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input
        .graphemes(true).rev().collect::<String>()
}


fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_result() {
        assert_eq!(reverse("123"),"321");
        assert_eq!(reverse("uüu"),"uüu");
    }
}
