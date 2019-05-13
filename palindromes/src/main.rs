use rayon::prelude::*;

fn est_palindrome(phrase: &str) -> bool {
    let length = phrase.len() / 2;
    phrase
        .chars()
        .take(length)
        .filter(|&c| !c.is_whitespace())
        .flat_map(|c| c.to_lowercase())
        .zip(
            phrase
                .chars()
                .rev()
                .take(length)
                .filter(|&c| !c.is_whitespace())
                .flat_map(|c| c.to_lowercase()),
        )
        .all(|(a, b)| a == b)
        && !phrase.is_empty()
}

fn tous_les_palindromes(phrases: &str) -> Vec<String> {
    phrases
        .split('.')
        .filter(|&p| est_palindrome(p))
        .map(|p| p.to_string())
        .collect()
}

fn tous_les_palindromes_en_parallele(phrases: &str) -> Vec<String> {
    phrases
        .par_split('.')
        .filter(|&p| est_palindrome(p))
        .map(|p| p.to_string())
        .collect()
}

fn main() {
    assert!(est_palindrome("un radar nu"));
    assert!(est_palindrome("Un radar nu"));
    assert_eq!(
        tous_les_palindromes("Un roc cornu. Foo Bar Baz. Un radar nu."),
        vec!["Un roc cornu", " Un radar nu"]
    );
    assert_eq!(
        tous_les_palindromes_en_parallele("Un roc cornu. Foo Bar Baz. Un radar nu."),
        vec!["Un roc cornu", " Un radar nu"]
    );
}
