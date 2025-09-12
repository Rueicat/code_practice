struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        // 容器
        fn val(c: char) -> i32 {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        }

        // init
        let mut total: i32 = 0;
        let mut max_right: i32 = 0;

        // loop
        for ch in s.chars().rev() {
            let v = val(ch);
            if v < max_right {
                total -= v;
            } else {
                total += v;
                max_right = v;
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
        assert_eq!(Solution::roman_to_int("XVII".to_string()), 17);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    }

}



fn main() {

}

