use axum::{
  extract::{Query, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
use serde::Deserialize;

use crate::users_db::UsersDb;

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
  pub offset: Option<usize>,
  pub limit: Option<usize>,
}

pub async fn get_all_users(
  pagination: Option<Query<Pagination>>,
  State(users_db): State<UsersDb>,
) -> impl IntoResponse {
  let users = users_db.read().unwrap();

  let Query(pagination) = pagination.unwrap_or_default();

  let users = users
    .values()
    .skip(pagination.offset.unwrap_or(0))
    .take(pagination.limit.unwrap_or(usize::MAX))
    .cloned()
    .collect::<Vec<_>>();

  (StatusCode::OK, Json(users))
}
