use std::collections::HashMap;

use warp::{hyper::StatusCode, Rejection, Reply};

use crate::{
    error,
    store::Store,
    types::{
        pagination::extract_pagination,
        question::{Question, QuestionId},
    },
};

pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
    id: String
) -> Result<impl Reply, Rejection> {
    log::info!("{} Start quering questions", id);
    let questions: Vec<Question> = store.questions.read().await.values().cloned().collect();
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        log::info!("{} Pagination set {:?}", id, &pagination);
        if pagination.end > questions.len() {
            return Err(warp::reject::custom(error::Error::EndParameterOutOfBounds));
        }
        let res = &questions[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        log::info!("{} No pagination used", id);
        Ok(warp::reply::json(&questions))
    }
}

pub async fn add_question(store: Store, question: Question) -> Result<impl Reply, Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);

    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

pub async fn update_question(
    id: String,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(error::Error::QuestionNotFound)),
    }

    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

pub async fn delete_question(
    id: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
        None => Err(warp::reject::custom(error::Error::QuestionNotFound)),
    }
}
