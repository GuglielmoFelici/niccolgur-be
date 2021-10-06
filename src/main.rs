#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::Route;
use rocket_contrib::databases::redis::Connection;

use controllers::*;

mod entities;
mod controllers;
#[allow(dead_code)]
mod services;
mod responses;
#[allow(dead_code)]
mod redis_keys;
mod views;

#[database("niccolgur_redis")]
pub struct NiccDbConn(Connection);

fn controllers() -> Vec<Route> {
    routes![
        /* HELLO */
        hello,
        pre_flight, // TODO elaborare...
        /* QUEUE */
        queue,
        queue_users,
        /* USER */
        user,
        users,
        users_full,
        /* AUTH */
        /*hello_protected,
        login,
        token_data,*/
        /* IMAGES */
        image,
        /* SEASONS */
        season,
        season_full,
        season_last,
        season_last_full,
        seasons_count
    ]
}

fn main() {
    rocket::ignite().attach(NiccDbConn::fairing()).mount("/", controllers()).launch();
}
