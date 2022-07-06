use std::ptr::read;
use md5::Digest;
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

fn increase(mut vec : Vec<u8>) -> Vec<u8>{
    for i in 0..8 {
        if vec[i] <= 254 {
            vec[i] = vec[i] + 1;
            return vec;
        }
    }
    return vec;
}






pub fn hash_cash(input: MD5HashCashInput) -> MD5HashCashOutput{
    //let mut vec : Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    //let mut vec2 : Vec<u8>;
    let mut hexa:u64 = 0;
    //hex::encode_upper(&vec);
    //let mut seed: &str;

    //let message = "adds";
    let message = input.message;

    
    let mut hash_code = md5::compute(message.clone());
    let mut its_good = false;

    while !its_good {
        let concat = format!("{hexa:016X}") + &*message; //&* pour cast en &str
        //println!("concat = {}", concat);

        hash_code = md5::compute(concat);
        //println!("hash_code = {:x}", hash_code.clone());

        let nb_zero = count_zero_at_the_beginning(format!("{:X}", hash_code.clone()));
        //println!("nb_zero = {:x}", nb_zero);

        if (nb_zero * 8) >= input.complexity as u8 {
            its_good = true;
        } else {
            //vec = increase(vec);
            //vec2 = vec.clone();
            //vec2.reverse();
            hexa = hexa + 1
            //caractere_en_plus + 1hexa
            // seed = seed + 1hexa

        }
    }
    let response = MD5HashCashOutput {
        seed: hexa.clone(),
        hashcode: format!("{:x}",hash_code)
    };
    print!("seed : {}, hash_code : {} \n",response.seed.clone(),response.hashcode.clone());
    return response;
}


#[cfg(test)]
mod tests {
    use common::{MD5HashCashInput, MD5HashCashOutput};
    use crate::count_zero_at_the_beginning;
    use crate::hash_cash::{hash_cash, increase};

    #[test]
    fn increase_tests() {
        let mut vec : Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0];
        vec = increase(vec);
        let mut shouldBe : Vec<u8> = vec![1, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(vec, shouldBe);

        let mut vec : Vec<u8> = vec![255, 255, 253, 0, 0, 0, 0, 0];
        vec = increase(vec);
        let mut shouldBe : Vec<u8> = vec![255, 255, 254, 0, 0, 0, 0, 0];
        assert_eq!(vec, shouldBe);

        let mut vec : Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255, 255];
        vec = increase(vec);
        let mut shouldBe : Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255, 255];
        assert_eq!(vec, shouldBe);
    }

    #[test]
    fn count_zero_at_the_beginning_test() {
        let value = count_zero_at_the_beginning("00dsfhgsdjhfgjqhfez".to_string());
        assert_eq!(value, 2);

        let value = count_zero_at_the_beginning("0000000000sfhgsdjhfgjqhfez".to_string());
        assert_eq!(value, 10);

        let value = count_zero_at_the_beginning("10000000000sfhgsdjhfgjqhfez".to_string());
        assert_eq!(value, 0);

        /*let value = count_zero_at_the_beginning("00000000000000".to_string()); //todo panic with that value
        assert_eq!(value, 14);*/

        let value = count_zero_at_the_beginning(" 0".to_string());
        assert_eq!(value, 0);
    }
    #[test]
    #[should_panic]
    fn count_zero_at_the_beginning_test_should_panic(){
        let value = count_zero_at_the_beginning("".to_string());
    }


    #[test]
    fn should_test_hash_cash(){
         let value = MD5HashCashInput{
            complexity: 2,
            message: "deuxzero".to_string()
        };
        let res = hash_cash(value);

        assert_eq!(res.hashcode, "027b29a689b2ab55f950e1504e604b47".to_string());
        assert_eq!(res.seed, 0x000000000000001D);
        let value = count_zero_at_the_beginning(res.hashcode);
        assert_eq!(value, 1);

        let value = MD5HashCashInput{
            complexity: 9,
            message: "message".to_string()
        };
        let res = hash_cash(value);
        assert_eq!(res.hashcode, "00aab01448e894ba0dbec511fc3afac2".to_string());
        assert_eq!(res.seed, 0x0000000000000028);
        let value = count_zero_at_the_beginning(res.hashcode);
        assert_eq!(value, 2);

        let value = MD5HashCashInput{
            complexity: 32,
            message: "message".to_string()
        };
        let res = hash_cash(value);
        assert_eq!(res.hashcode, "00005a8fa74ad2c7d1149bb2eb3fa954".to_string());
        assert_eq!(res.seed, 156138);
        let value = count_zero_at_the_beginning(res.hashcode);
        assert_eq!(value, 4);

        let value = MD5HashCashInput{
            complexity: 40,
            message: "message".to_string()
        };
        let res = hash_cash(value);
        assert_eq!(res.hashcode, "00000e8c9cdc0d11f9d5c2fc58af7b0c".to_string());
        assert_eq!(res.seed, 331932);
        let value = count_zero_at_the_beginning(res.hashcode);
        assert_eq!(value, 5);
    }
}
