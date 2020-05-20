#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod entities;
mod controllers;
mod services;
#[allow(dead_code)]
mod redis_keys;

use rocket_contrib::databases::redis::Connection;
use controllers::*;
use rocket::Route;

#[database("niccolgur_redis")]
pub struct NiccDbConn(Connection);

fn controllers() -> Vec<Route> {
    routes![
        /* HELLO */
        hello,
        /* USER */
        user,
        users,
        users_full,
    ]
}

fn main() {
    rocket::ignite().attach(NiccDbConn::fairing()).mount("/", controllers()).launch();
}
