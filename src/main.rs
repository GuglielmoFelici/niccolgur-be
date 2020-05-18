#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::ContentType;

use rocket_contrib::databases::redis::{self, Commands};

#[database("niccolgur_redis")]
struct NiccDb(redis::Connection);

struct User {
    id: String,
    nickname: String,
    bio: String,
}

impl<'r> Responder<'r> for User {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .ok()
    }
}

#[get("/user_id")]
fn user_id(conn: NiccDb) -> Option<String> {
    match conn.get("user:id") {
        Ok(id) => Some(id),
        Err(_) => None,
    }
}

fn main() {
    rocket::ignite().mount("/", routes![user_id]).launch();
}
