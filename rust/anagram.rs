use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
 //   todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let lower_word = word.to_lowercase();
    let sorted_word = sort_chars(&lower_word);

    //比較
    possible_anagrams
        .iter()
        .copied()
        .filter(|&candidate| {
            let lower_candidate = candidate.to_lowercase();

            lower_candidate != lower_word && sort_chars(&lower_candidate) == sorted_word
        })
        .collect()
}

fn sort_chars (s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();  //只是變成 iter, 散亂的
    chars.sort_unstable();            //類似 [] 才可排序
    chars.into_iter().collect()       //變回String
}

fn main() {}


#[cfg(test)]
mod test_one {
    use super::*;

    #[test]
    fn test_fn() {
        let word = "stone";
        let possible_anagrams = ["toesn", "bbbb", "wafjwaije","waijewea"];
        let result = anagrams_for(word, &possible_anagrams);
        
        let expected: HashSet<&str> = ["toesn"].iter().copied().collect();

        assert_eq!(result,expected);
    }
}

