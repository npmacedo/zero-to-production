use serde::Deserialize;
use secrecy::{Secret, ExposeSecret};

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        let username = &self.username;
        let password = &self.password.expose_secret();
        let host = &self.host;
        let port = self.port;
        let database_name = &self.database_name;
        Secret::new(format!("postgres://{username}:{password}@{host}:{port}/{database_name}"))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        let username = &self.username;
        let password = &self.password.expose_secret();
        let host = &self.host;
        let port = &self.port;
        Secret::new(format!("postgres://{username}:{password}@{host}:{port}"))
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();

    settings.merge(config::File::with_name("configuration"))?;

    settings.try_into()
}
