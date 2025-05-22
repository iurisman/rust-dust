use std::sync::Arc;
use config;
use crate::database::Postgres;

mod http_server;
mod database;
//mod error;

fn read_config() -> Result<config::Config, config::ConfigError> {
    config::Config::builder()
        .add_source(config::File::with_name("config.yaml"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
}

#[tokio::main]
async fn main() {
    let appState = AppState::init().await;
    let port = appState.config.get::<String>("port").unwrap();
    let local_addr = format!("127.0.0.1:{port}");
    let listener = tokio::net::TcpListener::bind(local_addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, http_server::router(appState)).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    database: Arc<Postgres>,
    config: config::Config,
}
impl AppState {
    pub async fn init() -> Self {
        let config: config::Config = read_config().unwrap();
        let mut pg_config: tokio_postgres::Config = tokio_postgres::Config::new();
        pg_config.host(config.get::<String>("postgres.host").unwrap());
        pg_config.password(config.get::<String>("postgres.password").unwrap());
        pg_config.user(config.get::<String>("postgres.user").unwrap());
        pg_config.password(config.get::<String>("postgres.password").unwrap());
        pg_config.dbname(config.get::<String>("postgres.dbname").unwrap());
        let foo = Postgres::new(pg_config).await;
        let database = Arc::new(foo);
        Self {database, config}
    }
}