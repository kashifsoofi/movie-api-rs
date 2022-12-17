use std::sync::Arc;

use super::store::{CreateMovieParams, DynMovieStore, Movie, MovieStore, Store, UpdateMovieParams};
use axum::async_trait;
use chrono::Utc;
use sqlx::{query_scalar, PgPool};
use uuid::Uuid;

pub struct SqlStore {
    db_pool: PgPool,
    movie_store: SqlMovieStore,
}

impl SqlStore {
    pub fn new(db_pool: PgPool) -> SqlStore {
        let movie_store = SqlMovieStore::new(db_pool.clone());
        Self {
            db_pool,
            movie_store,
        }
    }
}

#[async_trait]
impl Store for SqlStore {
    async fn is_connected(&self) -> bool {
        let connected = query_scalar!("SELECT TRUE;").fetch_one(&self.db_pool).await;
        let connected = match connected {
            Ok(connected) => connected.unwrap(),
            Err(_) => false,
        };

        connected
    }

    async fn movie_store(&self) -> DynMovieStore {
        Arc::new(self.movie_store.clone()) as DynMovieStore
    }
}

#[derive(Clone)]
pub struct SqlMovieStore {
    db_pool: PgPool,
}

impl SqlMovieStore {
    fn new(db_pool: PgPool) -> Self {
        SqlMovieStore { db_pool }
    }
}

#[async_trait]
impl MovieStore for SqlMovieStore {
    async fn get_all(&self) -> Vec<Movie> {
        let movies = sqlx::query_as!(
            Movie,
            r#"
            SELECT
                id, title, director, release_date, ticket_price, created_at, updated_at
            FROM movies
            "#
        )
        .fetch_all(&self.db_pool)
        .await
        .unwrap();

        movies
    }

    async fn get_by_id(&self, id: Uuid) -> Option<Movie> {
        let movie = sqlx::query_as!(
            Movie,
            r#"
            SELECT
                id, title, director, release_date, ticket_price, created_at, updated_at
            FROM movies
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.db_pool)
        .await
        .ok()?;

        movie
    }

    async fn create(&self, create_movie: CreateMovieParams) -> Result<Movie, String> {
        let movie = sqlx::query_as!(
            Movie,
            r#"
            INSERT INTO movies (id, title, director, release_date, ticket_price, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, title, director, release_date, ticket_price, created_at, updated_at
            "#,
            Uuid::new_v4(),
            create_movie.title,
            create_movie.director,
            create_movie.release_date,
            create_movie.ticket_price,
            Utc::now().naive_utc(),
            Utc::now().naive_utc()
        )
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(movie)
    }

    async fn update(&self, id: Uuid, movie_to_update: UpdateMovieParams) -> Result<Movie, String> {
        let movie = sqlx::query_as!(
            Movie,
            r#"
            SELECT
                id, title, director, release_date, ticket_price, created_at, updated_at
            FROM movies
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.db_pool)
        .await
        .unwrap();

        let title = match movie_to_update.title {
            Some(title) => title,
            _ => movie.title,
        };
        let director = match movie_to_update.director {
            Some(director) => director,
            _ => movie.director,
        };
        let release_date = match movie_to_update.release_date {
            Some(release_date) => release_date,
            _ => movie.release_date,
        };
        let ticket_price = match movie_to_update.ticket_price {
            Some(ticket_price) => ticket_price,
            _ => movie.ticket_price,
        };
        let movie = sqlx::query_as!(
            Movie,
            r#"
            UPDATE movies
            SET title = $2, 
                director = $3,
                release_date = $4,
                ticket_price = $5,
                updated_at = $6
            WHERE id = $1
            RETURNING id, title, director, release_date, ticket_price, created_at, updated_at
            "#,
            id,
            title,
            director,
            release_date,
            ticket_price,
            Utc::now().naive_utc()
        )
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(movie)
    }

    async fn delete(&self, id: Uuid) -> Result<Movie, String> {
        let movie = sqlx::query_as!(
            Movie,
            r#"
            DELETE FROM movies WHERE id = $1
            RETURNING id, title, director, release_date, ticket_price, created_at, updated_at
            "#,
            id
        )
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(movie)
    }
}
