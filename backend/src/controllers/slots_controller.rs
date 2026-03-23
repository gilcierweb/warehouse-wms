use actix_web::{delete, Error, get, HttpResponse, post, put, web};
use serde::Deserialize;
use uuid::Uuid;
use crate::db::database::Database;