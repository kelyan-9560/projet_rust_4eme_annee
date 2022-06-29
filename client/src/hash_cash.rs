use common::{MD5HashCashInput, MD5HashCashOutput};



pub fn count_zero_at_the_beginning(word: String) -> u8{
    let mut count = 0;

    for i in word.chars() {
        if i == '0' {
            count = count + 1;
        } else {
            return count;
        }
    }
    return panic!("mince");
}



pub fn hash_cash(input: MD5HashCashInput) -> MD5HashCashOutput{

    let seed = "";
    //let mut seed: &str;

    let adds = "adds";
    //let adds = input.message;

    
    let hash_code:String = "".to_string();
    let mut its_good = false;

    while !its_good {
        let concat = seed.to_string() + adds;
        println!("concat = {}", concat);

        let hash_code = md5::compute(concat);
        println!("hash_code = {:x}", hash_code);

        let nb_zero = count_zero_at_the_beginning(format!("{:X}", hash_code));
        println!("nb_zero = {:x}", nb_zero);

        if (nb_zero * 8) >= input.complexity as u8 {
            its_good = true;
        } else {
            //caractere_en_plus + 1hexa
            // seed = seed + 1hexa

        }
    }

    let response = MD5HashCashOutput {
        seed: seed.parse().unwrap(),
        hashcode: hash_code
    };

    return response;
}