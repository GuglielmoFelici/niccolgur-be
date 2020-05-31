#![feature(proc_macro_hygiene, decl_macro, try_trait)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::Route;
use rocket_contrib::databases::redis::Connection;

use controllers::*;

mod entities;
mod controllers;
mod services;
mod errors;
#[allow(dead_code)]
mod redis_keys;

#[database("niccolgur_redis")]
pub struct NiccDbConn(Connection);

fn controllers() -> Vec<Route> {
    routes![
        /* HELLO */
        hello,
        /* QUEUE */
        queue_users,
        /* USER */
        user,
        users,
        users_full,
        /* IMAGES */
        image,
        /* SEASONS */
        season,
        season_last,
        season_last_full
    ]
}

fn main() {
    rocket::ignite().attach(NiccDbConn::fairing()).mount("/", controllers()).launch();
}
