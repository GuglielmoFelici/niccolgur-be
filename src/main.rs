#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::collections::HashMap;
use std::ffi::NulError;
use std::io::Cursor;

use redis::*;
use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket_contrib::databases::redis::{self, Commands};

#[database("niccolgur_redis")]
struct NiccDbConn(redis::Connection);

struct User {
    id: String,
    nickname: String,
    bio: String,
}

impl<'r> Responder<'r> for User {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(format!("{}, {}, {}", self.id, self.nickname, self.bio)))
            .header(ContentType::new("text", "plain"))
            .ok()
    }
}

impl User {
    fn from_map(map: HashMap<String, String>) -> Option<User> {
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

#[get("/hello")]
fn hello() -> &'static str {
    "Hello cane!"
}

#[get("/user/<id>")]
fn user_id(conn: NiccDbConn, id: String) -> Option<User> {
    let user_map: HashMap<String, String> = match conn.hgetall(format!("user:{}", &id)) {
        Ok(map) => map,
        Err(E) => panic!(),
    };
    User::from_map(user_map)
}

fn main() {
    rocket::ignite().attach(NiccDbConn::fairing()).mount("/", routes![hello, user_id]).launch();
}
