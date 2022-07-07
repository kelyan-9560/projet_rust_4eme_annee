use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Welcome {
    pub version: u8
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    pub name: String
}

//////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}
//////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug)]
pub struct PublicPlayer{
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64
}

pub type PublicLeaderBoard = Vec<PublicPlayer>;
//////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeName {
    MD5HashCashInput,
    RecoverSecretInput
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeInput{
    challenge_input: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Challenge {
    challenge_name : ChallengeName
}

//////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug)]
pub struct  ChallengeAnswer {
    answer: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeResult {
    result: ChallengeAnswer,
    next_target: String
}
//////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
pub enum  ChallengeValue {
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportedChallengeResult {
    name: String,
    value : ChallengeValue
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

//////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug)]
pub struct EndOfGame {
    leader_board: PublicLeaderBoard
}


//////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashInput {
    // complexity in bits
    pub complexity: u32,
    // message to sign
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashOutput {
    // Seed used to solve the challenge
    pub seed: u64,
    // hashcode found using seed + message
    pub hashcode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecoverSecretInput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecoverSecretOutput {
    pub secret_sentence: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(PublicLeaderBoard),
    Challenge(ChallengeName),
    ChallengeResult(ChallengeResult),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame)
}


