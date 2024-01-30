use axum::{
  http::StatusCode,
  routing::{get, post},
  Json, Router,
};
use axum_hello_world::{
  users::{
    create_user::create_user, delete_user::delete_user, get_all_users::get_all_users,
    get_user::get_user, update_user::update_user,
  },
  users_db::UsersDb,
};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
  let users_db = UsersDb::default();

  let users_routes = Router::new()
    .route("/", post(create_user).get(get_all_users))
    .route("/:id", get(get_user).put(update_user).delete(delete_user))
    .with_state(users_db);

  let api = Router::new()
    .nest("/users", users_routes)
    .fallback(api_fallback);

  let app = Router::new().nest("/api", api);

  let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("127.0.0.1:8080")
    .await
    .unwrap();

  axum::serve(listener, app).await.unwrap();
}

async fn api_fallback() -> (StatusCode, Json<Value>) {
  let body = json!({
      "status": 404,
      "message": "Not Found"
  });

  (StatusCode::NOT_FOUND, Json(body))
}
