pub use config::ConfigError;
use serde::Deserialize;
use slog::{o, Drain, Logger};
use slog_async;
use slog_envlogger;
use slog_term;

#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::default().separator("."))
            .build()?;
        cfg.try_deserialize()
    }

    pub fn configure_log() -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
        let console_drain = slog_envlogger::new(console_drain);
        let console_drain = slog_async::Async::new(console_drain).build().fuse();
        slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
    }
}
