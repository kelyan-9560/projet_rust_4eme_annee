

/*
retrouver la phrase à partir d'un tuple
je reçois letters
un tableau de tailles de tuples
je peux construire les tuples
Condition: phrase valide si elle repond aux critères, pas quand elle
correspond exactement
Critères: nbre de mots, l'ordre des n-uplets des caractères

Entrée:
*nbre de mots de la phrase
*les lettres qui forment les tuples
*tuple-sizes:

Sortie:
Une phrase String qui contient word_count mots, avec ordre des
lettres respecté dans les tuples

Genre à partir des tuples
Il fait froid
Exemple [(i,f,f), (i,i,i), (l,f,a), (t,r,o), (r,i,d), (a,t,o)]
i => r =>
r => t
t => a
a => f => l

l f a t r i o
NB: complexité inférieur à 17, ignore word_count puisque de toute façon
Je renvoi une suite de caractères séparés par des espaces
 */

use common::RecoverSecretInput;

fn recover_secret(input: RecoverSecretInput) -> String {
    let word_count = input.word_count;
    let letters = input.letters;
    let tuple_sizes = input.tuple_sizes;

    //let mut sentence = Vec::new();
    //pseudo-code
    /*
    parse tuple_sizes, curseur sur lettres selon current contenu de tuple,
    let acc = 0;
    for i in 0..tuple_sizes.length {

        for j in acc..tuple_sizes[i] {

        }
        acc += tuple_sizes[i]
    }
     */
    let chars: Vec<char> = letters.chars().collect();
    let mut sentence = Vec::new();
    //sentence.push(chars[0]);
    let mut acc = 0;
    for i in 0..tuple_sizes.len() {
        for j in acc..(acc + tuple_sizes[i]) {
            //checkIfCarInSentence(letters[j])    , if not, sentence.push(sentence.push(letters[j]))
            if sentence.contains(&chars[j]) == false {
                sentence.push(chars[j]);
            }
        }
        acc += tuple_sizes[i];
    }

    let mut a_string = String::from("");
    for i in 0..sentence.len() {
        if(sentence[i] != ' '){
            a_string.push(sentence[i]);
            a_string.push(' ');
        }else {
            a_string.push(sentence[i]);
        }

    }

    return a_string;
}
