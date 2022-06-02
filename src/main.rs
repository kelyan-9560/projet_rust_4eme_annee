use std::arch::aarch64::float32x2_t;
use serde::{Serialize, Deserialize};


///
struct  Welcome {
    version: u8
}

struct Subscribe {
    name: String
}
///

struct Err {
    error: String
}



struct  SubscribeResult {
    message: String,
    err : Err(String)
}
///

struct PublicPlayer{
    name: String,
    stream_id: String,
    score: String,
    steps: i32,
    is_active: bool,
    total_used_time: f32
}

struct ListPlayers{
    players: Vec<PublicPlayer>

}
enum PublicLeaderBoard{
    ListPlayers
}
///

struct ChallengeName {
    name: String
}
struct Challenge {
    challenge: ChallengeName(ChallengeInput)
}
struct ChallengeInput{
    challenge_input: String
}

///
struct  ChallengeAnswer {
    answer: String
}
struct ChallengeResult {
    result: ChallengeAnswer,
    next_target: String
}
///

struct JobValue {

}

struct ReportedChallengeResult {
    name: String,
    value {}
}
struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

///

struct EndOfGame {

}


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
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);

}
