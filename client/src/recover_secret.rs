use common::RecoverSecretInput;

fn recover_secret(input: RecoverSecretInput) -> String {
    let word_count = input.word_count;
    let letters = input.letters;
    let tuple_sizes = input.tuple_sizes;

    let chars: Vec<char> = letters.chars().collect();
    let mut sentence = Vec::new();
    //sentence.push(chars[0]);
    let mut acc = 0;
    for i in 0..tuple_sizes.len() {
        for j in acc..(acc + tuple_sizes[i]) {
            if sentence.contains(&chars[j]) == false {
                sentence.push(chars[j]);
            }
        }
        acc += tuple_sizes[i];
    }

    return sentence_formatter(sentence);
}

fn sentence_formatter(sentence: Vec<char>) -> String{
    let mut a_string = String::from("");
    for i in 0..sentence.len() {
        a_string.push(sentence[i]);
        if(sentence[i] != ' '){
            a_string.push(' ');
        }
    }

    return a_string;
}
