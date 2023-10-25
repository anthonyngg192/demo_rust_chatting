use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;

use crate::{Error, Result};
use std::collections::BTreeMap;

pub fn routes() -> Router {
    Router::new().route("/api/login", post(login_api))
}

async fn login_api(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    print!("->> {:12} ", "LOGIN_API");

    if payload.username != "demo1" || payload.password != "Test123!" {
        return Err(Error::LoginFailed);
    }

    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();

    claims.insert("username", payload.username.to_string());
    claims.insert("password", payload.password.to_string());

    let token_str = claims.sign_with_key(&key).unwrap();

    Ok(Json(json!({
            "result":{
            "success":true,
            "token":token_str
        }
    })))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}
