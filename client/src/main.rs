extern crate core;

mod hash_cash;
mod recover_secret;

use common::*;
use std::io::{Read, Write};
use std::net::{TcpStream};
use std::string::ParseError;
use common::ChallengeAnswer::ChallengeName;
use crate::hash_cash::hash_cash;


fn send_message_to_server(mut stream: &TcpStream, message: Message){
    //struct -> json -> byte
    //serialiser le message
    let serialized_message = serde_json::to_string(&message).unwrap();
    print!("{:?}", serialized_message);

    let message_as_bytes = serialized_message.as_bytes();
    print!("{:?}", message_as_bytes);

    //prendre la taille (en u32) du message serialisé
    let size:u32 = serialized_message.len() as u32;
    print!("{:?}", size);

    // le convertir en bigendian avec la fonction to_be_bytes()
    //(Les chiffres sont réprésenté différement en fonction
    //du système d'exploitation, bigendian est la convention pour tout OS)
    let bigendian_size = size.to_be_bytes();
    print!("{:?}", bigendian_size);

    //Envoyer la taille
    stream.write(&bigendian_size).expect("Error: send size failed !");
    // puis le message
    stream.write(serialized_message.as_ref()).expect("Error: Send message failed !");
}

fn get_size_from_message(message: Vec<u8>) -> u8{
    let mut size_u32 = 0;
    for i in message {
        if i != 0 {
            size_u32 = i;
        }
    }
    return size_u32
}

fn server_message_reception(mut stream: &TcpStream) -> Message {
    //Receptionner la taille
    let mut size : Vec<u8> = vec![0; 10];
    let _read_result = &stream.read(&mut size); // size_result == est ce que ça s'est bien passé
                                                                    // le resultat du read est dans size
    println!("size : {:?}", size);

    let size_u32 = get_size_from_message(size);
    println!("size as u32 : {:?}", size_u32);

    //Receptionner le message
    let mut input_message : Vec<u8> = vec![0; size_u32 as usize];
    let mut _read_message = &stream.read(&mut input_message);
    println!("input_message {:?}", input_message);

    //      passer de l'octet au String
    let input_message_str = String::from_utf8_lossy(&input_message);
    println!("input_message_str {:?}", input_message_str);
    //       le deserialiser
    let deserialized_message: Message = serde_json::from_str(&input_message_str).unwrap();
    println!("deserialized = {:?}", deserialized_message);

    return deserialized_message
}

fn md5_output_to_challenge_result(md5_output: MD5HashCashOutput) -> Message {

    let challenge_output = ChallengeOutput::MD5HashCashOutput(md5_output);
    let challenge_answer = ChallengeAnswer::ChallengeName(challenge_output);
    let challenge_result = ChallengeResult{
        result: challenge_answer,
        next_target : String::from("jsp")
    };
    return Message::ChallengeResult(challenge_result);
}

fn recover_secret_output_to_challenge_result(recover_output: RecoverSecretOutput) -> Message {

    let challenge_output = ChallengeOutput::RecoverSecretOutput(recover_output);
    let challenge_answer = ChallengeAnswer::ChallengeName(challenge_output);
    let challenge_result = ChallengeResult{
        result: challenge_answer,
        next_target : String::from("jsp")
    };
    return Message::ChallengeResult(challenge_result);
}





fn main(){

    let name = std::env::args().nth(1).expect("no name");

    let stream = TcpStream::connect("localhost:7878");

    match stream {
        Ok(stream) => {

            let message = Message::Hello;

            send_message_to_server(&stream, message);

            //Welcome
            server_message_reception(&stream);

            let val = String::from(name);
            let message = Message::Subscribe(Subscribe{name: val});
            send_message_to_server(&stream, message);


            //SubscribeResult
            server_message_reception(&stream);

            //PublicLeaderBoard
            server_message_reception(&stream);

            //Challenge
            let message = server_message_reception(&stream);

/*
            match message {
                Message::Hello => {}
                Message::Welcome(_) => {}
                Message::Subscribe(_) => {}
                Message::SubscribeResult(_) => {}
                Message::PublicLeaderBoard(_) => {}

                Message::Challenge(c) => {
                    match c {
                        ChallengeName::MD5HashCash(md5Input) => {

                            let output = hash_cash(md5Input);
                            let md5_result = md5_output_to_challenge_result(output);

                            send_message_to_server(&stream, md5_result);
                        }

                        ChallengeName::RecoverSecret(recover_input) => {
                            let output = recover_secret(md5Input);
                            let recover_result = recover_secret_output_to_challenge_result(output);

                            send_message_to_server(&stream, md5_result);
                        }
                    }
                }
                Message::ChallengeResult(_) => {}
                Message::RoundSummary(_) => {}
                Message::EndOfGame(_) => {}
            }

 */
        }
        Err(err) => panic!("Cannot connect : {:?}", err)
    }

}