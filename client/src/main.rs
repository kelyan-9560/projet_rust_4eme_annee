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

            // serialiser le message
                //struct -> json -> byte
            let message = Message::Hello;
            let serialized_message = serde_json::to_string(&message).unwrap();
            print!("{:?}", serialized_message);

            let message_as_bytes = serialized_message.as_bytes();
            print!("{:?}", message_as_bytes);


            //prendre la taille (en u32) du message serialisé
            let size:u32 = serialized_message.len() as u32;
            print!("{:?}", size);

            // le convertir en bigendian avec la fonction to_be_bytes()
                //Les chiffres sont en réprésenté différement en fonction
                //du système d'exploitation, bigendian est la convention pour tout les OS
            let bigendian_size = size.to_be_bytes();
            print!("{:?}", bigendian_size);

            //envoyer la taille
            stream.write(&bigendian_size).expect("Error: send size failed !");
                // puis le message
            stream.write(serialized_message.as_ref()).expect("Error: Send message failed !");


            //faire le process inverse pour recevoir les messages

        }
        Err(err) => panic!("Cannot connect : {err}")
    }

}