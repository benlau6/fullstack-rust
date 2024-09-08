use dotenvy::dotenv;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::ConnectOptions;

#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub security: SecuritySettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub rust_log: String,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

#[derive(Deserialize)]
pub struct SecuritySettings {
    pub secret_key: String,
}

pub fn get_environment() -> Environment {
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .expect("APP_ENVIRONMENT not set in system environment variables.")
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");
    environment
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // - A base configuration file, for values that are shared across our local and production environment
    // (e.g. database name);
    // - A collection of environment-specific configuration files, specifying values for fields that require cus-
    // tomisation on a per-environment basis (e.g. host);
    // - An environment variable, APP_ENVIRONMENT, to determine the running environment (e.g. produc-
    // tion or local).
    // - All configuration files will live in the same top-level directory, configuration.

    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");
    // Detect the running environment.
    // load dotenv
    dotenv().expect("Failed to load .env file");
    let environment = get_environment();
    let environment_filename = format!("{}.toml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.toml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        // Add in settings from environment variables (with a prefix of APP and
        // '__' as separator)
        // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    // Try to convert the configuration values it read into
    // our Settings type
    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn with_db(&self) -> PgConnectOptions {
        let mut options = self.without_db().database(&self.database_name);
        options = options.log_statements(tracing_log::log::LevelFilter::Trace);
        options
    }
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            // Try an encrypted connection, fallback to unencrypted if it fails
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
}

/// The possible runtime environment for our application.
#[derive(PartialEq)]
pub enum Environment {
    Local,
    Development,
    Production,
}
impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Development => "development",
            Environment::Production => "production",
        }
    }
}
impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. \
    Use either `local`, `development`, or `production`.",
                other
            )),
        }
    }
}
