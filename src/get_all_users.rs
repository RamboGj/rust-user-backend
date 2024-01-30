use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::users_db::UsersDb;

pub async fn get_all_users(State(users_db): State<UsersDb>) -> impl IntoResponse {
  let users = users_db.read().unwrap();

  (StatusCode::CREATED, Json(users.clone()))
}
