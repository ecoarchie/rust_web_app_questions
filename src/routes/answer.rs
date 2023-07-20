use std::collections::HashMap;
use uuid::Uuid;
use warp::{http::StatusCode, Rejection, Reply};

use crate::{
    error,
    store::Store,
    types::{
        answer::{Answer, AnswerId},
        question::QuestionId,
    },
};

pub async fn add_answer(
    store: Store,
    params: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    let content = match params.get("content") {
        Some(c) => c.to_string(),
        None => return Err(warp::reject::custom(error::Error::MissingParameters)),
    };

    let question_id = match params.get("questionId") {
        Some(id) => QuestionId(id.to_string()),
        None => return Err(warp::reject::custom(error::Error::MissingParameters)),
    };

    match store.questions.read().await.get(&question_id) {
        Some(_) => {
            let answer = Answer {
                id: AnswerId(Uuid::new_v4()),
                content,
                question_id,
            };

            store
                .answers
                .write()
                .await
                .insert(answer.id.clone(), answer);
        }
        None => return Err(warp::reject::custom(error::Error::QuestionNotFound)),
    };
    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}
