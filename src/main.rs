use movie_api::configuration::get_configuration;
use movie_api::telemetry::{get_subscriber, init_subscriber};

use movie_api::startup::Application;

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("movie-api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let applicatin = Application::build(configuration)
        .await
        .expect("Failed to build application");
    applicatin.run_until_stopped().await;
}
