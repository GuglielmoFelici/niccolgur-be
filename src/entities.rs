use rocket::response::{self, Responder, Response};
use rocket::request::Request;
use std::io::Cursor;
use rocket::http::ContentType;
use serde::Serialize;
use std::collections::HashMap;


#[derive(Debug, Serialize, Hash, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub nickname: String,
    pub bio: String,
}

impl User {
    pub fn from_map(map: &HashMap<String, String>) -> Option<User> {
        let nickname = map.get("nickname")?.to_string();
        let id = map.get(&nickname)?.to_string();
        let bio = map.get("bio")?.to_string();
        Option::from(User {
            nickname,
            id,
            bio,
        })
    }
}

impl<'r> Responder<'r> for User { // TODO
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(format!("{:?}", self)))
            .header(ContentType::new("text", "plain"))
            .ok()
    }
}