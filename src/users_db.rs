use serde::Serialize;
use std::{
  collections::HashMap,
  sync::{Arc, RwLock},
};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct User {
  pub id: Uuid,
  pub name: String,
  pub username: String,
}

pub type UsersDb = Arc<RwLock<HashMap<Uuid, User>>>;
