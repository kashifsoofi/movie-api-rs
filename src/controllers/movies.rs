use std::str::FromStr;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::store::store::{CreateMovieParams, DynMovieStore, UpdateMovieParams};

#[derive(Deserialize, Serialize)]
pub struct MovieResponse {
    id: Uuid,
    title: String,
    director: String,
    release_date: String,
    ticket_price: f64,
    created_at: String,
    updated_at: String,
}

pub async fn list(State(movie_store): State<DynMovieStore>) -> impl IntoResponse {
    let movies = movie_store.get_all().await;
    let mut movie_responses = Vec::new();
    for movie in movies {
        let movie_response = MovieResponse {
            id: movie.id,
            title: movie.title,
            director: movie.director,
            release_date: movie.release_date.to_rfc3339(),
            ticket_price: movie.ticket_price,
            created_at: movie.created_at.to_rfc3339(),
            updated_at: movie.updated_at.to_rfc3339(),
        };
        movie_responses.push(movie_response);
    }

    (StatusCode::OK, Json(movie_responses))
}

pub async fn get(
    Path(id): Path<Uuid>,
    State(movie_store): State<DynMovieStore>,
) -> Result<Json<MovieResponse>, AppError> {
    let movie = movie_store.get_by_id(id).await;
    match movie {
        None => {
            return Err(AppError::MovieNotFound);
        }
        Some(movie) => {
            let movie_response = MovieResponse {
                id: movie.id,
                title: movie.title,
                director: movie.director,
                release_date: movie.release_date.to_rfc3339(),
                ticket_price: movie.ticket_price,
                created_at: movie.created_at.to_rfc3339(),
                updated_at: movie.updated_at.to_rfc3339(),
            };

            return Ok(Json(movie_response));
        }
    }
}

// the input to our `create` handler
#[derive(Deserialize)]
pub struct CreateMovieRequest {
    title: String,
    director: String,
    release_date: String,
    ticket_price: f64,
}

pub async fn create(
    State(movie_store): State<DynMovieStore>,
    Json(request): Json<CreateMovieRequest>,
) -> Result<Json<MovieResponse>, AppError> {
    let release_date = DateTime::from_str(&request.release_date);
    let release_date = match release_date {
        Ok(release_date) => release_date,
        Err(_) => {
            return Err(AppError::ValidationError(
                "Invalid release date".to_string(),
            ))
        }
    };

    let params = CreateMovieParams {
        title: request.title,
        director: request.director,
        release_date: release_date,
        ticket_price: request.ticket_price,
    };
    let movie = movie_store.create(params).await;
    let movie = match movie {
        Ok(movie) => movie,
        Err(error_message) => return Err(AppError::Unknown(error_message)),
    };

    let movie_response = MovieResponse {
        id: movie.id,
        title: movie.title,
        director: movie.director,
        release_date: movie.release_date.to_rfc3339(),
        ticket_price: movie.ticket_price,
        created_at: movie.created_at.to_rfc3339(),
        updated_at: movie.updated_at.to_rfc3339(),
    };
    Ok(movie_response.into())
}

#[derive(Deserialize)]
pub struct UpdateMovieRequest {
    title: Option<String>,
    director: Option<String>,
    release_date: Option<String>,
    ticket_price: Option<f64>,
}

pub async fn update(
    Path(id): Path<Uuid>,
    State(movie_store): State<DynMovieStore>,
    Json(request): Json<UpdateMovieRequest>,
) -> Result<Json<MovieResponse>, AppError> {
    let release_date = match request.release_date {
        None => None,
        Some(release_date) => {
            let release_date = DateTime::from_str(&release_date);
            let release_date = match release_date {
                Err(_) => {
                    return Err(AppError::ValidationError(
                        "Invalid release date".to_string(),
                    ))
                }
                Ok(release_date) => release_date,
            };
            Some(release_date)
        }
    };

    let params = UpdateMovieParams {
        title: request.title,
        director: request.director,
        release_date: release_date,
        ticket_price: request.ticket_price,
    };
    let movie = movie_store.update(id, params).await;
    let movie = match movie {
        Ok(movie) => movie,
        Err(error_message) => return Err(AppError::Unknown(error_message)),
    };

    let movie_response = MovieResponse {
        id: movie.id,
        title: movie.title,
        director: movie.director,
        release_date: movie.release_date.to_rfc3339(),
        ticket_price: movie.ticket_price,
        created_at: movie.created_at.to_rfc3339(),
        updated_at: movie.updated_at.to_rfc3339(),
    };
    Ok(movie_response.into())
}

pub async fn delete(
    Path(id): Path<Uuid>,
    State(movie_store): State<DynMovieStore>,
) -> Result<Json<MovieResponse>, AppError> {
    let movie = movie_store.delete(id).await;
    match movie {
        Err(_) => return Err(AppError::MovieNotFound),
        Ok(movie) => {
            let movie_response = MovieResponse {
                id: movie.id,
                title: movie.title,
                director: movie.director,
                release_date: movie.release_date.to_rfc3339(),
                ticket_price: movie.ticket_price,
                created_at: movie.created_at.to_rfc3339(),
                updated_at: movie.updated_at.to_rfc3339(),
            };

            return Ok(movie_response.into());
        }
    }
}

pub enum AppError {
    MovieNotFound,
    ValidationError(String),
    Unknown(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::MovieNotFound => (StatusCode::NOT_FOUND, "Movie not found"),
            AppError::ValidationError(_error_message) => {
                (StatusCode::BAD_REQUEST, "validation error")
            }
            AppError::Unknown(_error_message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "unknown error")
            }
        };

        let body = Json(json!({ "error_message": error_message }));
        (status, body).into_response()
    }
}
