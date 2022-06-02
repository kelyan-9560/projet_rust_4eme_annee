use std::io::Read;
//use std::arch::aarch64::float32x2_t;
use std::net::{SocketAddr, TcpListener};
use serde::{Serialize, Deserialize};


///
#[derive(Serialize, Deserialize, Debug)]
struct  Welcome {
    version: u8
}

#[derive(Serialize, Deserialize, Debug)]
struct Subscribe {
    name: String
}
///

#[derive(Serialize, Deserialize, Debug)]
struct Err {
    error: String
}


#[derive(Serialize, Deserialize, Debug)]
struct  SubscribeResult {
    message: String,
    err : Err
}
///
#[derive(Serialize, Deserialize, Debug)]
struct PublicPlayer{
    name: String,
    stream_id: String,
    score: String,
    steps: i32,
    is_active: bool,
    total_used_time: f32
}
#[derive(Serialize, Deserialize, Debug)]
struct ListPlayers{
    players: Vec<PublicPlayer>

}

#[derive(Serialize, Deserialize, Debug)]
enum PublicLeaderBoard{
    ListPlayers
}
///

#[derive(Serialize, Deserialize, Debug)]
struct ChallengeName {
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ChallengeInput{
    challenge_input: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Challenge {
    challenge_name : ChallengeName
}


///
///
#[derive(Serialize, Deserialize, Debug)]
struct  ChallengeAnswer {
    answer: String
}
#[derive(Serialize, Deserialize, Debug)]
struct ChallengeResult {
    result: ChallengeAnswer,
    next_target: String
}
///

#[derive(Serialize, Deserialize, Debug)]
struct JobValue {

}

#[derive(Serialize, Deserialize, Debug)]
struct ReportedChallengeResult {
    name: String,
    value : JobValue
}

#[derive(Serialize, Deserialize, Debug)]
struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

///
#[derive(Serialize, Deserialize, Debug)]
struct EndOfGame {
    leader_board: PublicLeaderBoard
}

/*
{
"RoundSummary":
    {
        "challenge":"MD5HashCash",
        "chain":
            [
                {
                    "name":"free_patato",
                    "value":
                        {
                            "Ok":
                                {
                                    "used_time":0.1,
                                    "next_target":"dark_salad"
                                }
                        }
                },
                {
                    "name":"dark_salad",
                    "value":"Unreachable"
                }
            ]
    }
}

*/



trait ChallengeBis {
    // Données en entrée du challenge
    type Input;
    // Données en sortie du challenge
    type Output;
    // Nom du challenge
    fn name() -> String;
    // Résout le challenge
    fn solve(&self) -> Self::Output;
    // Vérifie qu'une sortie est valide pour le challenge
    fn verify(&self, answer: Self::Output) -> bool;
}



#[derive(Serialize, Deserialize, Debug)]
enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(PublicLeaderBoard),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame)
}




#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {

    let message = Message::Welcome(Welcome{version: 1});

    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

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
