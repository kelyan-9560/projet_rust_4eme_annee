use std::net::{SocketAddr, TcpListener};
//use crate::common


fn main() {

    let message = Message::Welcome(Welcome{version: 1});

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&message).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Message = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);



    // Connexion à un serveur
    let address = SocketAddr::from(([127, 0, 0, 1], 7676));

    let listener = TcpListener::bind(address);

    let listener = match listener {
        Ok(l) => l,
        Err(err) => panic!("Cannot connect : {err}")
    };


    //Interprétation des messages
    for message in listener.incoming(){
        println!("message = {message:?}");
        let mut input_message = message.unwrap();

        let mut v = Vec::<u8>::new();

        input_message.read_to_end(&mut v).expect("TODO: panic message");

        let str = String::from_utf8_lossy(&v); // d'octet vers String


        //Gérer les messages
        // si str == 'Hello
        //      do_something()
        // ect...
    }

}