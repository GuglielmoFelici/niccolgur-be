use std::io::Cursor;
use std::str::FromStr;

use rocket::{Request, response, Response};
use rocket::http::{Accept, Header};
use rocket::http::hyper::header::{Authorization, Bearer};
use rocket::response::Responder;
use rocket_contrib::json::Json;
use serde::export::fmt::Display;
use serde::Serialize;

use crate::entities::User;
use std::fmt::Formatter;
use core::fmt;

#[derive(Debug, Serialize, Hash, PartialEq, Eq)]
pub struct UserView {
    pub username: String,
    pub bio: String,
    pub id: String,
}

impl From<User> for UserView {
    fn from(usr: User) -> UserView {
        UserView {
            username: usr.nickname,
            bio: usr.bio,
            id: usr.id,
        }
    }
}

impl From<&User> for UserView {
    fn from(usr: &User) -> UserView {
        UserView {
            username: (*usr.nickname).parse().unwrap(),
            bio: (*usr.bio).parse().unwrap(),
            id: (*usr.id).to_owned(),
        }
    }
}

#[derive(Serialize)]
pub struct TokenView {
    pub token: String,
    pub expiration: usize,
}

// impl<'r> Responder<'r> for LoginResponse {
//     fn respond_to(self, _: &Request) -> response::Result<'r> {
//         Response::build()
//             .sized_body(Cursor::new(json!((self.data)).to_string()))
//             .raw_header(
//                 "Authorization",
//                 self.token,
//             )
//             .ok()
//     }
// }