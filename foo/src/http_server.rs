use axum::Router;
use axum::routing::{get, post};
use axum::response::Html;
use axum::{http::StatusCode, response::IntoResponse};
use axum::extract;
use axum::handler::Handler;
use http::Response;
use serde::{Serialize, Deserialize};
use super::database::Postgres;
use tokio::runtime::Handle;
use crate::AppState;

pub fn router(app_state: AppState) -> Router {
   Router::new()
       .route("/",
              get(health)
                  .post(save_beat))
       .with_state(app_state)
}

async fn health() -> Html<&'static str> {
   println!("Getting /");
   Html("Health")
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Beat {
   pub customer_id: String,
   pub event_count: i32,
}

async fn save_beat(
   extract::State(state): extract::State<AppState>,
   extract::Json(beat): extract::Json<Beat>
) -> Result<String, StatusCode> {
   println!("Saving /Beat {beat:?}");
   match state.database.save_beat(&beat).await {
      Ok(n) => Ok(format!("Successfully saved {n} rows")),
      Err(error) => {
         eprintln!("Error: {error:?}");
         Err(StatusCode::INTERNAL_SERVER_ERROR)
      },
   }

}

#[cfg(test)]
mod tests {
   use super::*;
   use axum_test::TestServer;
   use tokio::sync::OnceCell;
   use http::status::StatusCode;

   async fn init_server() -> &'static TestServer {
      async fn new_server() -> TestServer {
         TestServer::new(router(AppState::init().await)).unwrap()
      }
      static SERVER: OnceCell<TestServer> = OnceCell::const_new();
      SERVER.get_or_init(|| new_server()).await
   }


   #[tokio::test]
   async fn test_beat() {
      println!("In test_beat");
      let server = init_server().await;
      let body = Beat{customer_id: String::from("1234ABC"), event_count:10000};
      let resp = server.post("/").json(&body).await;
      resp.assert_status(StatusCode::OK);
      println!("resp: {:?}", resp);
   }
   #[tokio::test]
   async fn test_health() {
      println!("In test_health");
      let server = init_server().await;
      let resp = server.get("/").await;
      resp.assert_status(StatusCode::OK);
      assert_eq!("Health", resp.text());
   }
}