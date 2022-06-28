use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct  Welcome {
    pub(crate) version: u8
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    name: String
}

///
#[derive(Serialize, Deserialize, Debug)]
pub struct Err {
    error: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct  SubscribeResult {
    message: String,
    err : Err
}
///
#[derive(Serialize, Deserialize, Debug)]
pub struct PublicPlayer{
    name: String,
    stream_id: String,
    score: String,
    steps: i32,
    is_active: bool,
    total_used_time: f32
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ListPlayers{
    players: Vec<PublicPlayer>

}

#[derive(Serialize, Deserialize, Debug)]
pub enum PublicLeaderBoard{
    ListPlayers
}
///

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeName {
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeInput{
    challenge_input: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Challenge {
    challenge_name : ChallengeName
}


///
///
#[derive(Serialize, Deserialize, Debug)]
pub struct  ChallengeAnswer {
    answer: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeResult {
    result: ChallengeAnswer,
    next_target: String
}
///

#[derive(Serialize, Deserialize, Debug)]
pub struct JobValue {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportedChallengeResult {
    name: String,
    value : JobValue
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

///
#[derive(Serialize, Deserialize, Debug)]
pub struct EndOfGame {
    leader_board: PublicLeaderBoard
}


#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
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



fn main() {
    println!("Hello, world!");
}
