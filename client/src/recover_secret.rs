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


#[cfg(test)]
mod tests {
    use common::RecoverSecretInput;
    use crate::recover_secret::recover_secret;

    #[test]
    fn should_test_recover_secret(){
        let value = RecoverSecretInput{
            word_count: 2,
            letters: "t cCehuCethoCeschouC'schout h".to_string(),
            tuple_sizes: vec![3, 4, 5, 7, 7, 3]
        };
        let res = recover_secret(value);
        println!("{}",res);

        assert_eq!(res,"C'est chou");
    }

    #[test]
    fn should_test_recover_secret2(){
        let value = RecoverSecretInput{
            word_count : 1,
            letters : "WvyOAlxafUzleiSOl9xayBeHTmy9xWTU5lMW4nUO5lMWRajn2BiHSRUzy5afnUz5wlexWrm5wlBWr4mAlBrUmzHxTUzwlHrfTwBeSRmzlMSRfoUOAe9S4oUiraOiramzM5w3l".to_string(),
            tuple_sizes : vec![6, 8, 4, 6, 4, 7, 8, 9, 6, 9, 8, 7, 5, 7, 6, 6, 9, 5, 4, 5, 4]
        };
        let res = recover_secret(value);
        println!("{}",res);

        assert_eq!(res,"xWRvraj4fonTUmzyO25wA3lBeiM9H");
    }
}