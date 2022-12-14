use uuid::Uuid;
use sqlx::PgPool;
use super::store::{Movie, MovieStore, Store, CreateMovieParams, UpdateMovieParams};

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

impl Store for SqlStore {
    fn movie_store(self) -> Box<dyn MovieStore> {
        Box::new(self.movie_store)
    }
}

pub struct SqlMovieStore {
    db_pool: PgPool,
}

impl SqlMovieStore {
    fn new(db_pool: PgPool) -> Self {
        SqlMovieStore {
            db_pool,
        }
    }
}

impl MovieStore for SqlMovieStore {
    fn get_all(&self) -> Vec<Movie> {
        todo!()
    }

    fn get_by_id(&self, id: Uuid) -> Option<Movie> {
        todo!()
    }

    fn create(&self, movie: CreateMovieParams) -> Result<Movie, String> {
        todo!()
    }

    fn update(&self, id: Uuid, movie_to_update: UpdateMovieParams) -> Result<Movie, String> {
        todo!()
    }

    fn delete(&self, id: Uuid) -> Result<Movie, String> {
        todo!()
    }
}