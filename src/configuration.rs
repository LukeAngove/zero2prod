use secrecy::ExposeSecret;
use secrecy::Secret;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_directory =
        std::env::current_dir().expect("Failed to determine the current directory.");
    let configuration_directory = base_directory.join("configuration");
    let basename = std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "local".into());
    let filename = format!("{}.yml", basename);
    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("base.yml")))
        .add_source(config::File::from(configuration_directory.join(filename)))
        .build()?;

    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        )
        .into()
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        )
        .into()
    }
}
