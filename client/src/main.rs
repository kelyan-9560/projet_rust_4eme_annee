extern crate core;

mod hash_cash;

use common::{Message, Subscribe};
use std::io::{Read, Write};
use std::net::{TcpStream};
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};




fn send_message_to_server(mut stream: &TcpStream, message: Message){
    // serialiser le message
    //struct -> json -> byte
    //let message = Message::Hello;
    let serialized_message = serde_json::to_string(&message).unwrap();
    print!("{:?}", serialized_message);

    let message_as_bytes = serialized_message.as_bytes();
    print!("{:?}", message_as_bytes);

    //prendre la taille (en u32) du message serialisé
    let size:u32 = serialized_message.len() as u32;
    print!("{:?}", size);

    // le convertir en bigendian avec la fonction to_be_bytes()
    //Les chiffres sont réprésenté différement en fonction
    //du système d'exploitation, bigendian est la convention pour tout les OS
    let bigendian_size = size.to_be_bytes();
    print!("{:?}", bigendian_size);

    //envoyer la taille
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

fn server_message_reception(mut stream: &TcpStream){
    //convertir le taille bigendian -> u32 -> usize

    //Receptionner la taille
    let mut size : Vec<u8> = vec![0; 10];
    let mut read_result = &stream.read(&mut size); // size_result == est ce que ça s'est bien passé
    // le resultat du read est dans size
    println!("size : {:?}", size);
    let mut size_u32 = get_size_from_message(size);
    //     convertir de bigendian à u32 (voir byteorder dependencies)
    //  let mut size_u32 = size.read_u32::<BigEndian>();
    //let mut size_u32 = size as u32;
    //  let mut size_u32 = size.to_le_bytes();
    //  let mut size_u32 = read_result.unwrap();
    //  let mut size_u32 = String::from_utf8_lossy(&*size);
    println!("size as u32 : {:?}", size_u32);

    //Receptionner le message
    let mut input_message : Vec<u8> = vec![0; size_u32 as usize];
    let mut read_message = &stream.read(&mut input_message);
    println!("input_message {:?}", input_message);

    //mettre le message dans le vecteur de la taille de size
    //let mut message_vector = vec![&input_message; &size[0] as usize];

    //      passer de l'octet au String
    let input_message_str = String::from_utf8_lossy(&input_message);
    println!("input_message_str {:?}", input_message_str);
    //       le deserialiser
    let deserialized_message: Message = serde_json::from_str(&input_message_str).unwrap();
    println!("deserialized = {:?}", deserialized_message);
}



fn main(){

    let stream = TcpStream::connect("localhost:7878");

    match stream {
        Ok(mut stream) => {

            let message = Message::Hello;
            send_message_to_server(&stream, message);

            println!("-------------------");

            //TODO : Récupérer les messages envoyés par le serveur

/*
            //convertir le taille bigendian -> u32 -> usize

            //Receptionner la taille
            let mut size : Vec<u8> = vec![0; 10];
            let mut read_result = &stream.read(&mut size); // size_result == est ce que ça s'est bien passé
                                                                  // le resultat du read est dans size
            println!("size : {:?}", size);
            let mut size_u32 = 0;
            for i in size {
                if i != 0 { 
                    size_u32 = i;
                }
            }
            //     convertir de bigendian à u32 (voir byteorder dependencies)
            //  let mut size_u32 = size.read_u32::<BigEndian>();
            //let mut size_u32 = size as u32;
            //  let mut size_u32 = size.to_le_bytes();
            //  let mut size_u32 = read_result.unwrap();
            //  let mut size_u32 = String::from_utf8_lossy(&*size);
            println!("size as u32 : {:?}", size_u32);

            //Receptionner le message
            let mut input_message : Vec<u8> = vec![0; size_u32 as usize];
            let mut read_message = &stream.read(&mut input_message);
            println!("input_message {:?}", input_message);

            //mettre le message dans le vecteur de la taille de size
            //let mut message_vector = vec![&input_message; &size[0] as usize];

            //      passer de l'octet au String
            let input_message_str = String::from_utf8_lossy(&input_message);
            println!("input_message_str {:?}", input_message_str);
            //       le deserialiser
            let deserialized_message: Message = serde_json::from_str(&input_message_str).unwrap();
            println!("deserialized = {:?}", deserialized_message);
*/

            server_message_reception(&stream);

            let val = String::from("Kélyan");
            let message = Message::Subscribe(Subscribe{name: val});
            send_message_to_server(&stream, message);

            //      l'interpreter
            // si str == 'Hello
            //      do_something()
            // ect...

        }
        Err(err) => panic!("Cannot connect : {:?}", err)
    }

}