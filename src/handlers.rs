use warp::{
    http::StatusCode,
    reject::{self, Rejection},
    reply, Reply,
};

use crate::{
    models::{Answer, GuessError, Letter, LetterState, Word},
    words::{VALID_WORDS, WORDS},
};

pub async fn guess(mut guess: String, answer: Answer) -> Result<impl Reply, Rejection> {
    if guess.len() != 5 {
        return Err(reject::custom(GuessError::InvalidLength));
    }

    if !guess.chars().all(|charecter| charecter.is_alphabetic()) {
        return Err(reject::custom(GuessError::InvalidCharacters));
    }

    guess = guess.to_uppercase();
    if !VALID_WORDS.contains(&guess.as_str()) && !WORDS.contains(&guess.as_str()) {
        return Err(reject::custom(GuessError::NotInWordList));
    }

    let answer = answer.read().unwrap();
    Ok(reply::json(
        &Word::try_from(
            guess
                .chars()
                .enumerate()
                .map(|(index, charecter)| Letter {
                    charecter,
                    state: if answer.contains(charecter) {
                        if answer.chars().nth(index).unwrap() == charecter {
                            LetterState::CorrectIndex
                        } else {
                            LetterState::InWord
                        }
                    } else {
                        LetterState::NotInWord
                    },
                })
                .collect::<Vec<Letter>>(),
        )
        .unwrap(),
    ))
}

pub async fn guess_rejection(rejection: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = rejection.find::<GuessError>() {
        Ok(reply::with_status(
            match error {
                GuessError::InvalidLength => "Invalid length",
                GuessError::InvalidCharacters => "Invalid characters",
                GuessError::NotInWordList => "Not in word list",
            },
            StatusCode::BAD_REQUEST,
        ))
    } else {
        Err(rejection)
    }
}
