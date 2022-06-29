use std::hash::Hash;
use common::{MD5HashCashInput, MD5HashCashOutput};
use md5::{Digest};

fn hash_cash(input: MD5HashCashInput){

    let its_good = false;
    let mut adds = "";
    let mut res = "";


    while !its_good {

        res = &*[adds, &input.message].join("");



        let hash_code = md5::compute(res);


        println!("{:x}", digest(hash_code));

        //print!("{:?}", hash_code)

    }





}