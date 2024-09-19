use serde::Serialize;
use std::sync::{Arc, RwLock};
use warp::reject;

const WORD_LENGTH: usize = 5;

pub type Answer = Arc<RwLock<String>>;

pub type Word = [Letter; WORD_LENGTH];

#[derive(Debug)]
pub enum GuessError {
    InvalidLength,
    InvalidCharacters,
    NotInWordList,
}

impl reject::Reject for GuessError {}

#[derive(Serialize, Debug)]
pub enum LetterState {
    NotInWord,
    InWord,
    CorrectIndex,
}

#[derive(Debug, Serialize)]
pub struct Letter {
    pub charecter: char,
    pub state: LetterState,
}
