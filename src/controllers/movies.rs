use std::str::FromStr;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::store::store::{CreateMovieParams, DynMovieStore, Movie, UpdateMovieParams};

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

impl From<Movie> for MovieResponse {
    fn from(movie: Movie) -> Self {
        MovieResponse {
            id: movie.id,
            title: movie.title,
            director: movie.director,
            release_date: movie.release_date.to_string(),
            ticket_price: movie.ticket_price.to_f64().unwrap(),
            created_at: movie.created_at.to_string(),
            updated_at: movie.updated_at.to_string(),
        }
    }
}

pub async fn list(State(movie_store): State<DynMovieStore>) -> impl IntoResponse {
    let movies = movie_store.get_all().await;
    let movie_responses: Box<[MovieResponse]> = movies.into_iter().map(Into::into).collect();

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
            let movie_response = MovieResponse::from(movie);
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

impl From<CreateMovieRequest> for CreateMovieParams {
    fn from(request: CreateMovieRequest) -> Self {
        CreateMovieParams {
            title: request.title,
            director: request.director,
            release_date: NaiveDateTime::from_str(&request.release_date).unwrap(),
            ticket_price: BigDecimal::from_f64(request.ticket_price).unwrap(),
        }
    }
}

pub async fn create(
    State(movie_store): State<DynMovieStore>,
    Json(request): Json<CreateMovieRequest>,
) -> Result<Json<MovieResponse>, AppError> {
    let release_date = NaiveDateTime::from_str(&request.release_date);
    let release_date = match release_date {
        Ok(release_date) => release_date,
        Err(_) => {
            return Err(AppError::ValidationError(
                "Invalid release date".to_string(),
            ))
        }
    };

    let movie = movie_store.create(request.into()).await;
    let movie = match movie {
        Ok(movie) => movie,
        Err(error_message) => return Err(AppError::Unknown(error_message)),
    };

    let movie_response = MovieResponse::from(movie);
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
            let release_date = NaiveDateTime::from_str(&release_date);
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

    let ticket_price = match request.ticket_price {
        None => None,
        Some(ticket_price) => BigDecimal::from_f64(ticket_price),
    };

    let params = UpdateMovieParams {
        title: request.title,
        director: request.director,
        release_date: release_date,
        ticket_price: ticket_price,
    };
    let movie = movie_store.update(id, params).await;
    let movie = match movie {
        Ok(movie) => movie,
        Err(error_message) => return Err(AppError::Unknown(error_message)),
    };

    let movie_response = MovieResponse::from(movie);
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
            let movie_response = MovieResponse::from(movie);
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
