use std::{collections::HashMap, sync::Arc};

use chrono::Utc;
use parking_lot::RwLock;
use uuid::Uuid;

use super::store::{Movie, MovieStore, Store, CreateMovieParams, UpdateMovieParams};

type Movies = HashMap<Uuid, Movie>;

#[derive(Clone)]
pub struct MemoryStore {
    movie_store: MemoryMovieStore    
}

impl MemoryStore {
    pub fn new() -> MemoryStore {
        let movie_store = MemoryMovieStore::new();
        Self {
            movie_store,
        }
    }
}

impl Store for MemoryStore {
    fn movie_store(self) -> Box<dyn MovieStore> {
        Box::new(self.movie_store)
    }
}

#[derive(Clone)]
pub struct MemoryMovieStore {
    movies: Arc<RwLock<Movies>>
}

impl MemoryMovieStore {
    fn new() -> Self {
        Self {
            movies: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl MovieStore for MemoryMovieStore {
    fn get_all(&self) -> Vec<Movie> {
        let mut result = Vec::new();
        let r = self.movies.read();

        for (_, value) in r.iter() {
            result.push((*value).clone());
        }

        result
    }

    fn get_by_id(&self, id: Uuid) -> Option<Movie> {
        let r = self.movies.read();
        let movie = r.get(&id);

        match movie {
            None => None,
            Some(movie) => Some((*movie).clone())
        }
    }

    fn create(&self, movie_to_create: CreateMovieParams) -> Result<Movie, String> {
        let movie = Movie {
            id: Uuid::new_v4(),
            title: movie_to_create.title,
            director: movie_to_create.director,
            release_date: movie_to_create.release_date,
            ticket_price: movie_to_create.ticket_price,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.movies.write().insert(movie.id, movie.clone());

        Ok(movie)
    }

    fn update(&self, id: Uuid, movie_to_update: UpdateMovieParams) -> Result<Movie, String> {
        let movie = self.get_by_id(id);
        let movie = match movie {
            None => return Err("not found".to_string()),
            Some(movie) => movie,
        };

        self.movies.write().entry(movie.id).and_modify(|m| {
            m.title = movie_to_update.title;
            m.director = movie_to_update.director;
            m.release_date =  movie_to_update.release_date;
            m.ticket_price = movie_to_update.ticket_price;
            m.updated_at =  Utc::now();
        });

        Ok(self.get_by_id(movie.id).unwrap())
    }

    fn delete(&self, id: Uuid) -> Result<Movie, String> {
        let r = self.movies.read();
        let movie = r.get(&id);
        match movie {
            None => Err("not found".to_string()),
            Some(movie) => {
                self.movies.write().remove(&id);
                Ok((*movie).clone())
            }
        }
    }
}