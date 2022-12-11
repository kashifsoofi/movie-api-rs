use config::Environment;
use serde_aux::prelude::deserialize_number_from_string;
use std::env;

#[derive(Clone, serde::Deserialize)]
pub struct Configuration {
    pub http_server: HttpServerConfiguration,
    pub database: DatabaseConfiguration,
}

#[derive(Clone, serde::Deserialize)]
pub struct HttpServerConfiguration
{
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

#[derive(Clone, serde::Deserialize)]
pub struct DatabaseConfiguration
{
    pub database_url: String,
    pub log_level: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub max_open_connections: u16,
}

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let current_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = current_path.join("configuration");

    let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".into());

        // Initialise our configuration reader
        let configuration = config::Config::builder()
        // Add configuration values from a file named `base.yaml`.
        .add_source(config::File::from(
            configuration_directory.join("default.yaml"),
        ))
        .add_source(
            config::File::from(configuration_directory.join(format!("{}.yaml", environment)))
                .required(false),
        )
        // Add in a local configuration file
        // This file shouldn't be checked in to git or source control
        .add_source(config::File::from(configuration_directory.join("local.yaml")).required(false))
        .add_source(Environment::default().separator("_"))
        .build()?;
    // Try to convert the configuration values it read into our configuration type
    configuration.try_deserialize::<Configuration>()
}