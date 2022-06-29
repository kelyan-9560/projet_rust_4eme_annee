use common::{MD5HashCashInput, MD5HashCashOutput};
extern crate hex;


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
    let mut vec : Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let mut vec2 : Vec<u8>;

    let mut hexa = hex::encode_upper(&vec);
    //let mut seed: &str;

    let message = "adds";
    //let message = input.message;

    
    let hash_code:String = "".to_string();
    let mut its_good = false;

    while !its_good {
        let concat = hexa.to_string() + message;
        println!("concat = {}", concat);

        let hash_code = md5::compute(concat);
        println!("hash_code = {:x}", hash_code);

        let nb_zero = count_zero_at_the_beginning(format!("{:X}", hash_code));
        println!("nb_zero = {:x}", nb_zero);

        if (nb_zero * 8) >= input.complexity as u8 {
            its_good = true;
        } else {
            vec = increase(vec);
            vec2 = vec.clone();
            vec2.reverse();
            hexa = hex::encode_upper(vec2);
            //caractere_en_plus + 1hexa
            // seed = seed + 1hexa
            
        }
    }

    let response = MD5HashCashOutput {
        seed: hexa.parse().unwrap(),
        hashcode: hash_code
    };

    return response;
}

fn increase(mut vec : Vec<u8>) -> Vec<u8>{
    for i in 0..8 {
        if vec[i] <= 254 {
            vec[i] = vec[i] + 1;
            return vec;
        }
    }
    return vec;
}