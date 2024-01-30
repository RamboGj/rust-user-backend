use axum::{
  extract::{Path, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
use serde::Serialize;
use uuid::Uuid;

use crate::users_db::UsersDb;

#[derive(Serialize)]
pub struct GetUserDTO {
  name: String,
  username: String,
}

pub async fn get_user(
  Path(id): Path<Uuid>,
  State(users_db): State<UsersDb>,
) -> Result<impl IntoResponse, StatusCode> {
  let users = users_db.read().unwrap();

  if let Some(user) = users.get(&id).cloned() {
    let user_dto = GetUserDTO {
      name: user.name,
      username: user.username,
    };

    Ok((StatusCode::ACCEPTED, Json(user_dto)))
  } else {
    Err(StatusCode::NOT_FOUND)
  }
}
