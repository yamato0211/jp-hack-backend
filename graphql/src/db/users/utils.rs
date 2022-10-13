pub struct Jwt;

use crate::db::users::{User};
use dotenv::dotenv;
use jsonwebtoken::{encode, EncodingKey};
use chrono::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    exp: i64,
    user_id: String
}

impl Jwt {
    pub fn encode_jwt(user: &User) -> String {
        dotenv().ok();
        let secret = dotenv::var("SECRET").unwrap();
        let mut header = jsonwebtoken::Header::default();
        header.alg = jsonwebtoken::Algorithm::HS256;
        let claim = Claims {
            exp: (chrono::Utc::now() + Duration::hours(8)).timestamp(),
            user_id: user.id.to_string(),
        };
        encode(&header, &claim, &EncodingKey::from_secret(secret.as_ref())).unwrap()
    }

    pub fn decode_jwt(jwt_token: &String) -> Option<String> {
        let secret = dotenv::var("SECRET").unwrap();
        let validation = jsonwebtoken::Validation::default();
        match jsonwebtoken::decode::<Claims>(
            &jwt_token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
            &validation,
        ){
            Ok(c) => Option::Some(c.claims.user_id),
            _ => Option::None
        }
    }   
}