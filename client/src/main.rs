extern crate core;

mod hash_cash;

use common::{Message, Welcome};
use std::io::Write;
use std::net::TcpStream;
use serde::{Serialize, Deserialize};
use crate::hash_cash::count_zero_at_the_beginning;


fn main(){

    //se connecter
    let stream = TcpStream::connect("localhost:7878");

    match stream {
        Ok(mut stream) => {

            //Envoyer un msg Hello, Welcome, Subscribe au serveur
                //La transformer en octets avec .as_bytes();

            //let message = Message::Hello.as_bytes();



            //convertir le msg en octets
            let message = "Hello".as_bytes();
            // le serialiser
            let serialized = serde_json::to_string(&message).unwrap();

            //prendre la taille (en u32) du message serialisé
            let size:u32 = serialized.len() as u32;

            // le convertir en bigendian avec la fonction to_be_bytes()
                //Les chiffres sont en réprésenté différement en fonction
                //du système d'exploitation, bigendian est la convention pour tout les OS
            let bigendian_size = size.to_be_bytes();

            //envoyer la taille
                // puis le message
            //let response = stream.write_all(serialized);


            //faire le process inverse pour recevoir les messages

        }
        Err(err) => panic!("Cannot connect : {err}")
    }


/*
    println!("------------");
    let test = String::from("0000g0reen");
    println!("{}", count_zero_at_the_beginning(test));

 */


}