use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use uuid::Uuid;

use crate::users_db::{User, UsersDb};

#[derive(Deserialize)]
pub struct CreateUserDTO {
  name: String,
  username: String,
}

pub async fn create_user(
  State(users_db): State<UsersDb>,
  Json(create_user_dto): Json<CreateUserDTO>,
) -> impl IntoResponse {
  let user = User {
    id: Uuid::new_v4(),
    name: create_user_dto.name,
    username: create_user_dto.username,
  };

  users_db.write().unwrap().insert(user.id, user.clone());

  (StatusCode::CREATED, Json(user))
}
