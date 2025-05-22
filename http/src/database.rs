use tokio_postgres;
use tokio_postgres::{Client, Connection, NoTls, Socket};
use crate::http_server::Beat;
use tokio::runtime::Handle;
use futures;
use tokio_postgres::tls::NoTlsStream;

pub struct Postgres {
    client: Client,
}

impl Postgres {
    pub async fn new(config: tokio_postgres::Config) -> Self {
        let (client, connection) =
            match config.connect(NoTls).await {
                Ok(c) => c,
                Err(e) => {
                    println!{"{}", e}
                    panic!("Connection could not be established because of error: {:?}", e)
                }
            };

        if let Err(e) = connection.await {
            eprintln!("connection error = {:?}", e)
        }

        Postgres{client}
    }

    pub async fn save_beat(&self, beat: &Beat) -> Result<u64, tokio_postgres::Error> {
        let timestamp = std::time::SystemTime::now();
        self.client.execute(
            "INSERT INTO beats(created_on, customer_id, event_count) \
            VALUES ($1, $2, $3) RETURNING id",
            &[&timestamp, &beat.customer_id, &beat.event_count],
        ).await
    }

}