use std::sync::Arc;

use axum::async_trait;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use uuid::Uuid;

pub type DynStore = Arc<dyn Store + Send + Sync>;

#[async_trait]
pub trait Store {
    async fn is_connected(&self) -> bool;
    async fn movie_store(&self) -> DynMovieStore;
}

pub type DynMovieStore = Arc<dyn MovieStore + Send + Sync>;

#[async_trait]
pub trait MovieStore {
    async fn get_all(&self) -> Vec<Movie>;
    async fn get_by_id(&self, id: Uuid) -> Option<Movie>;
    async fn create(&self, movie: CreateMovieParams) -> Result<Movie, String>;
    async fn update(&self, id: Uuid, movie: UpdateMovieParams) -> Result<Movie, String>;
    async fn delete(&self, id: Uuid) -> Result<Movie, String>;
}

#[derive(Clone, Debug)]
pub struct Movie {
    pub id: Uuid,
    pub title: String,
    pub director: String,
    pub release_date: NaiveDateTime,
    pub ticket_price: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct CreateMovieParams {
    pub title: String,
    pub director: String,
    pub release_date: NaiveDateTime,
    pub ticket_price: BigDecimal,
}

pub struct UpdateMovieParams {
    pub title: Option<String>,
    pub director: Option<String>,
    pub release_date: Option<NaiveDateTime>,
    pub ticket_price: Option<BigDecimal>,
}
