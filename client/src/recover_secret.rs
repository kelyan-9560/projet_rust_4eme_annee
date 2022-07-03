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
