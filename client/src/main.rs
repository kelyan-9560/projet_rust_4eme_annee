use std::io::Write;
use std::net::TcpStream;

fn main(){
    let stream = TcpStream::connect("localhost:7676");

    match stream {
        Ok(mut stream) => {

            // Serialize la structure


            //La transformer en octets avec .as_bytes();
            let messages = "Hello".as_bytes();
            let response = stream.write_all(messages);
        }
        Err(err) => panic!("Cannot connect : {err}")
    }
}