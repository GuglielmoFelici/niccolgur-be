use std::collections::HashMap;
use serde::Serialize;
use crate::errors::{EntityError, EntityResult};


#[derive(Debug, Serialize, Hash, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub nickname: String,
    pub bio: String,
}

impl User {
    pub fn from_map(map: &HashMap<String, String>) -> EntityResult<User> {
        let nickname = entity_key(&map, "nickname")?;
        let id = entity_key(&map, &nickname)?;
        let bio = entity_key(&map, "bio")?;
        Ok(User {
            nickname,
            id,
            bio,
        })
    }
}

#[derive(Debug, Serialize, Hash, PartialEq, Eq)]
pub struct Niccolgur {
    pub master: String,
    pub movie_id: String,
    pub members: Vec<String>,
    pub date: String,
}

impl Niccolgur {
    pub fn from_map(map: &HashMap<String, String>) -> EntityResult<Niccolgur> {
        Ok(Niccolgur {
            master: entity_key(&map, "master")?,
            movie_id: entity_key(&map, "movie")?,
            members: vec![],
            date: entity_key(&map, "date")?,
        })
    }
}

fn entity_key(map: &HashMap<String, String>, key: &str) -> EntityResult<String> {
    Ok(map.get(key).ok_or(EntityError)?.to_string())
}