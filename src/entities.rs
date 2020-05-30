use std::collections::HashMap;
use std::io::Cursor;

use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde::Serialize;


#[derive(Debug, Serialize, Hash, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub nickname: String,
    pub bio: String,
}

impl User {
    pub fn from_map(map: &HashMap<String, String>) -> Result<User, String> {
        let nickname = map.get("nickname").ok_or("bio field not present")?.to_string();
        let id = map.get(&nickname).ok_or(format!("{} field not present", nickname))?.to_string();
        let bio = map.get("bio").ok_or("bio field not present")?.to_string();
        Ok(User {
            nickname,
            id,
            bio,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct Niccolgur {
    master: String,
    movie_id: String,
    members: Vec<String>,
    date: String,
}

// impl<'r> Responder<'r> for User { // TODO
//     fn respond_to(self, _: &Request) -> response::Result<'r> {
//         Response::build()
//             .sized_body(Cursor::new(format!("{:?}", self)))
//             .header(ContentType::new("text", "plain"))
//             .ok()
//     }
// }