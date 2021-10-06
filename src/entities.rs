use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::responses::{TransformError, TransformResult};

#[derive(Debug, Serialize, Hash, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub nickname: String,
    pub bio: String,
    pub password: String,
}

// TODO usare costanti per chiavi
impl User {
    pub fn from(map: HashMap<String, String>) -> TransformResult<User> {
        let nickname = entity_key(&map, "nickname")?;
        let id = entity_key(&map, &nickname)?;
        let bio = entity_key(&map, "bio")?;
        let password = entity_key(&map, "password")?;
        Ok(User {
            nickname,
            id,
            bio,
            password,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Hash, PartialEq, Eq)]
pub struct Niccolgur {
    pub master: String,
    pub movie_id: String,
    pub members: Vec<String>,
    pub date: String,
}

impl Niccolgur {
    pub fn from(map: &HashMap<String, String>) -> TransformResult<Niccolgur> {
        Ok(Niccolgur {
            master: entity_key(&map, "master")?,
            movie_id: entity_key(&map, "movie")?,
            members: vec![],
            date: entity_key(&map, "date")?,
        })
    }
}

fn entity_key(map: &HashMap<String, String>, key: &str) -> TransformResult<String> {
    Ok(map.get(key).ok_or(TransformError)?.to_string())
}