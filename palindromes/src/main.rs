use rayon::prelude::*;

fn est_palindrome(phrase: &str) -> bool {
    let rev_phrase: String = phrase.chars().into_iter().rev().collect();
    phrase.to_lowercase().replace(" ", "") == rev_phrase.to_lowercase().replace(" ", "")
}

fn tous_les_palindromes(phrases: &str) -> Vec<String> {
    let palindromes: Vec<String> = phrases
        .split(".")
        .map(|x| x.to_string())
        .filter(|x| est_palindrome(x) && !x.is_empty())
        .collect();
    palindromes
}

fn tous_les_palindromes_en_parallele(phrases: &str) -> Vec<String> {
    let palindromes: Vec<String> = phrases
        .split(".")
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .into_par_iter()
        .filter(|x| est_palindrome(x) && !x.is_empty())
        .collect();
    palindromes
}

fn main() {
    assert!(est_palindrome("un radar nu"));
    assert!(est_palindrome("Un radar nu"));
    assert!(est_palindrome("Un roc cornu"));
    assert!(!est_palindrome("Toto"));
    assert_eq!(
        tous_les_palindromes("Un roc cornu. Foo Bar Baz. Un radar nu."),
        vec!["Un roc cornu", " Un radar nu"]
    );
    assert_eq!(
        tous_les_palindromes_en_parallele("Un roc cornu. Foo Bar Baz. Un radar nu."),
        vec!["Un roc cornu", " Un radar nu"]
    );
}
