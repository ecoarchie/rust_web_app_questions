#![warn(clippy::all)]

use warp::{http::Method, Filter};

mod error;
mod routes;
mod store;
mod types;

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    log::error!("This is an error!");
    log::info!("This is info");
    log::warn!("This is a warning!");

    let log = warp::log::custom(|info| {
        log::info!(
            "{} {} {} {:?} from {} with {:?}\n",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
            info.remote_addr().unwrap(),
            info.request_headers()
        );
    });

    let store = store::Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let id_filter = warp::any().map(|| uuid::Uuid::new_v4().to_string());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and(id_filter)
        .and_then(routes::question::get_questions);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(routes::answer::add_answer);

    let routes = get_questions
        .or(add_question)
        .or(add_answer)
        .or(update_question)
        .or(delete_question)
        .with(cors)
        .with(log)
        .recover(error::return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
