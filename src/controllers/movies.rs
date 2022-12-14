use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
struct MovieResponse {
    id: Uuid,
    title: String,
    director: String,
    release_date: String,
    ticket_price: f64,
}

fn create_test_movie() -> MovieResponse {
    MovieResponse {
        id: Uuid::new_v4(),
        title: Uuid::new_v4().to_string(),
        director: Uuid::new_v4().to_string(),
        release_date: Utc::now().to_rfc3339(),
        ticket_price: 10.99,
    }
}

pub async fn list() -> impl IntoResponse {
    let mut movies: Vec<MovieResponse> = Vec::new();
    movies.push(create_test_movie());
    movies.push(create_test_movie());

    (StatusCode::OK, Json(movies))
}

pub async fn get(Path(id): Path<Uuid>) -> impl IntoResponse {
    let movie_response = MovieResponse {
        id,
        title: String::from("Title"),
        director: String::from("Director"),
        release_date: Utc::now().to_rfc3339(),
        ticket_price: 10.99,
    };

    (StatusCode::OK, Json(movie_response))
}

// the input to our `create` handler
#[derive(Deserialize)]
pub struct CreateMovieRequest {
    title: String,
    director: String,
    release_date: String,
    ticket_price: f64,
}

pub async fn create(Json(request): Json<CreateMovieRequest>) -> impl IntoResponse {
    let movie_response = MovieResponse {
        id: Uuid::new_v4(),
        title: request.title,
        director: request.director,
        release_date: request.release_date,
        ticket_price: request.ticket_price,
    };

    (StatusCode::CREATED, Json(movie_response))
}

#[derive(Deserialize)]
pub struct UpdateMovieRequest {
    title: String,
    director: String,
    release_date: String,
    ticket_price: f64,
}

pub async fn update(Json(request): Json<UpdateMovieRequest>) -> impl IntoResponse {
    let movie_response = MovieResponse {
        id: Uuid::new_v4(),
        title: request.title,
        director: request.director,
        release_date: request.release_date,
        ticket_price: request.ticket_price,
    };

    (StatusCode::OK, Json(movie_response))
}

pub async fn delete(Path(_id): Path<Uuid>) -> impl IntoResponse {
    StatusCode::NO_CONTENT
}
