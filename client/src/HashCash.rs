use std::hash::Hash;
use common::{MD5HashCashInput, MD5HashCashOutput};
use md5::{Digest};

fn hash_cash(input: MD5HashCashInput) {
    let mut its_good = false;
    let adds = "adds";
    let mut res = "res";


    while !its_good {
        let concat = res.to_string() + adds;
        println!("concat = {}", concat);

        let hash_code = md5::compute(concat);
        println!("hash_code = {:x}", hash_code);


        //compter les zero au debut du hash_code

        /*
            if (zero * 8 >= complexite ){
                cestbon = true
                //(res, hash);
            }
        */


        //caractere_en_plus + 1hexa


        its_good = true;
    }
}