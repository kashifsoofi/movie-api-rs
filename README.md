# Movie API (Rust)

This is a small project to learn developing rest api in Rust with [axum](https://github.com/tokio-rs/axum) and [SQLx](https://github.com/launchbadge/sqlx) inspired by [article series by Stephen Walther](http://stephenwalther.com/archive/2015/01/12/asp-net-5-and-angularjs-part-1-configuring-grunt-uglify-and-angularjs). Goal of the project is to develop a simple rest api that would serve a `/health` endpoint and CRUD endpoints for a single resource `Movie`. There would be no authentication/authorisation.

This would also demonstrate how to run sql migration scripts using [sqlx cli](https://github.com/launchbadge/sqlx). This can be replaced with any other database migration tool that you want to learn and evaluate e.g. I have used [RoundhousE](https://github.com/chucknorris/roundhouse) in the past.

## Start Development Environment
Development dependencies can be started with `docker-compose`. This would start up PostgreSQL server and create and run docker image to run database migration on PostgreSQL server.
```
docker-compose -f docker-compose.dev-env.yml
```
This is only needed if you are going to run api against PostgreSQL. There is an in memory store available in code, that can be used instead of PostgreSQL.

## Run Project
### To run with in memory store
- set `store_type: memory` in configuration/default.yaml under database
- run with `cargo run`
### To run with Postgres store
- set `store_type: sql` in configuration/default.yaml under database
- run with `cargo run`

## API Endpoints
- GET `/health`
- GET `/movies` list all movies
- POST `/movies` create a new movie
- GET `/movies/{id}` get movie by id
- PUT `/movies/{id}` update a movie
- DELETE `/movies/{id}` delete a movie

## Resource
This not a most acurate representation of how you would model a movie resource in an acutal system, just a mix of few basic types and how to handle those in rest api.
### Movie
| Field       | Type    |
|-------------|---------|
| ID          | UUID    |
| Title       | String  |
| Director    | String  |
| ReleaseDate | Time    |
| TicketPrice | float64 |

## Test
There is an [Insomnia Document](https://github.com/kashifsoofi/movie-api-go/blob/main/Insomnia-Document.json) in the repository that can be used to test the api with [Insomnia Rest Client](https://insomnia.rest/).

## References
- [Building ASP.NET 5 apps with AngularJS](http://stephenwalther.com/archive/2015/01/12/asp-net-5-and-angularjs-part-1-configuring-grunt-uglify-and-angularjs)
- [How I write HTTP services after eight years.](https://pace.dev/blog/2018/05/09/how-I-write-http-services-after-eight-years.html)
- [How to write a Go API: The Ultimate Guide](https://jonnylangefeld.com/blog/how-to-write-a-go-api-the-ultimate-guide)
- [axum](https://github.com/tokio-rs/axum)
- [SQLx](https://github.com/launchbadge/sqlx) 
