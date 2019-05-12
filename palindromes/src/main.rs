use rayon::prelude::*;

fn est_palindrome(phrase: &str) -> bool {
    let rev_phrase: String = phrase.chars().into_iter().rev().collect();
    // println!(
    //     "Phrase:     {}\nRev phrase: {}\n",
    //     retire_espace_accents_ponctuation(&rev_phrase),
    //     retire_espace_accents_ponctuation(&phrase)
    // );
    retire_espace_accents_ponctuation(&phrase) == retire_espace_accents_ponctuation(&rev_phrase)
}

fn retire_espace_accents_ponctuation(phrase: &str) -> String {
    phrase
        .to_lowercase()
        .replace(" ", "")
        .replace(",", "")
        .replace("'", "")
        .replace("é", "e")
        .replace("è", "e")
        .replace("ë", "e")
        .replace("ê", "e")
        .replace("à", "a")
        .replace("â", "a")
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
    assert!(est_palindrome("Mon nom"));
    assert!(est_palindrome("Eh ca va la vache"));
    assert!(est_palindrome("À l'émir, Asimov a vomi sa rime, là"));
    assert!(est_palindrome("Engage le jeu que je le gagne"));
    assert!(est_palindrome("Noël a trop par rapport à Léon"));
    assert!(est_palindrome("À l'étape, épate la"));
    assert!(est_palindrome("La mère Gide digère mal"));
    assert!(est_palindrome("Léon, émir cornu, d'un roc rime Noël"));
    assert!(est_palindrome("Élu par cette crapule"));
    assert!(est_palindrome("La mariée ira mal"));
    assert!(est_palindrome("Le ruban à Burel"));
    assert!(est_palindrome("Été le bar arabe l'été"));
    assert!(est_palindrome("Tâte l'État"));
    assert!(est_palindrome("Un roc cornu"));
    assert!(est_palindrome(
        "Tu l'as trop écrasé  César  ce Port Salut"
    ));
    assert!(est_palindrome("rue Verlaine gela le génial rêveur"));
    assert!(est_palindrome("Et la marine va venir à Malte"));
    assert!(est_palindrome("La malade pédala mal"));
    assert!(est_palindrome("Elle dira hélas à la sale haridelle"));

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
