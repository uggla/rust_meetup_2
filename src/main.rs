use rayon::prelude::*;

fn est_palindrome(phrase: &str) -> bool {
    unimplemented!()
}

fn tous_les_palindromes(phrases: &str) -> Vec<String> {
    unimplemented!()
}

fn tous_les_palindromes_en_parallele(phrases: &str) -> Vec<String> {
    unimplemented!()
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
