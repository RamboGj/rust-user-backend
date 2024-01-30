use axum::{
  extract::{Path, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};

use serde::Deserialize;
use uuid::Uuid;

use crate::users_db::UsersDb;

#[derive(Deserialize)]
pub struct UpdateUserDTO {
  name: Option<String>,
  username: Option<String>,
}

pub async fn update_user(
  Path(id): Path<Uuid>,
  State(users_db): State<UsersDb>,
  Json(dto): Json<UpdateUserDTO>,
) -> Result<impl IntoResponse, StatusCode> {
  let mut user = users_db
    .read()
    .unwrap()
    .get(&id)
    .cloned()
    .ok_or(StatusCode::NOT_FOUND)?;

  if let Some(name) = dto.name {
    user.name = name;
  }

  if let Some(username) = dto.username {
    user.username = username;
  }

  users_db.write().unwrap().insert(user.id, user.clone());

  Ok((StatusCode::CREATED, Json(user)))
}
