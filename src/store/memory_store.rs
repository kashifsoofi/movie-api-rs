use std::{collections::HashMap, sync::Arc};

use axum::async_trait;
use chrono::Utc;
use parking_lot::RwLock;
use uuid::Uuid;

use super::store::{CreateMovieParams, DynMovieStore, Movie, MovieStore, Store, UpdateMovieParams};

type Movies = HashMap<Uuid, Movie>;

#[derive(Clone)]
pub struct MemoryStore {
    movie_store: MemoryMovieStore,
}

impl MemoryStore {
    pub fn new() -> MemoryStore {
        let movie_store = MemoryMovieStore::new();
        Self { movie_store }
    }
}

#[async_trait]
impl Store for MemoryStore {
    async fn is_connected(&self) -> bool {
        true
    }

    async fn movie_store(&self) -> DynMovieStore {
        Arc::new(self.movie_store.clone()) as DynMovieStore
    }
}

#[derive(Clone)]
pub struct MemoryMovieStore {
    movies: Arc<RwLock<Movies>>,
}

impl MemoryMovieStore {
    fn new() -> Self {
        Self {
            movies: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl MovieStore for MemoryMovieStore {
    async fn get_all(&self) -> Vec<Movie> {
        let mut result = Vec::new();
        let r = self.movies.read();

        for (_, value) in r.iter() {
            result.push((*value).clone());
        }

        result
    }

    async fn get_by_id(&self, id: Uuid) -> Option<Movie> {
        let r = self.movies.read();
        let movie = r.get(&id);

        match movie {
            None => None,
            Some(movie) => Some((*movie).clone()),
        }
    }

    async fn create(&self, movie_to_create: CreateMovieParams) -> Result<Movie, String> {
        let movie = Movie {
            id: Uuid::new_v4(),
            title: movie_to_create.title,
            director: movie_to_create.director,
            release_date: movie_to_create.release_date,
            ticket_price: movie_to_create.ticket_price,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        self.movies.write().insert(movie.id, movie.clone());

        Ok(movie)
    }

    async fn update(&self, id: Uuid, movie_to_update: UpdateMovieParams) -> Result<Movie, String> {
        let movie = self.get_by_id(id).await;
        let movie = match movie {
            None => return Err("not found".to_string()),
            Some(movie) => movie,
        };

        self.movies.write().entry(movie.id).and_modify(|m| {
            match movie_to_update.title {
                Some(title) => {
                    m.title = title;
                    m.updated_at = Utc::now().naive_utc();
                }
                _ => {}
            }
            match movie_to_update.director {
                Some(director) => {
                    m.director = director;
                    m.updated_at = Utc::now().naive_utc();
                }
                _ => {}
            }
            match movie_to_update.release_date {
                Some(release_date) => {
                    m.release_date = release_date;
                    m.updated_at = Utc::now().naive_utc();
                }
                _ => {}
            }
            match movie_to_update.ticket_price {
                Some(ticket_price) => {
                    m.ticket_price = ticket_price;
                    m.updated_at = Utc::now().naive_utc();
                }
                _ => {}
            }
        });

        Ok(self.get_by_id(movie.id).await.unwrap())
    }

    async fn delete(&self, id: Uuid) -> Result<Movie, String> {
        let movie = self.movies.write().remove(&id);
        match movie {
            None => Err("not found".to_string()),
            Some(movie) => Ok(movie),
        }
    }
}
