use std::collections::HashSet;
use std::error::Error;

use rocket::http::ContentType;
use rocket::response::Content;
use rocket_contrib::databases::r2d2_redis::redis::RedisError;
use rocket_contrib::json::Json;

use crate::entities::*;
use crate::NiccDbConn;
use crate::services::{self};

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello cane laido!"
}

/********************************************* Queue ***********************************************
***************************************************************************************************/

#[get("/queue")]
pub fn queue(conn: &NiccDbConn) -> Json<Vec<String>> {
    Json(services::queue(&conn)?)
}

#[get("/queue/full")]
pub fn queue_users(conn: NiccDbConn) -> Result<Json<Vec<User>>, impl Error> {
    services::queue_users(&conn).map(Json)
}

/********************************************* Users ***********************************************
***************************************************************************************************/


#[get("/users")]
pub fn users(conn: NiccDbConn) -> Result<Json<HashSet<String>>, impl Error> {
    services::users(&conn).map(Json)
}

#[get("/users/full")]
pub fn users_full(conn: NiccDbConn) -> Result<Json<HashSet<User>>, impl Error> {
    services::users_full(&conn).map(Json)
}

#[get("/users/id/<id>")]
pub fn user(conn: NiccDbConn, id: String) -> Result<Json<User>, impl Error> {
    services::user(&conn, &id).map(Json)
}


/********************************************* Images **********************************************
***************************************************************************************************/

#[get("/images/id/<id>")]
pub fn image(conn: NiccDbConn, id: String) -> Content<Result<Vec<u8>, impl Error>> {
    Content(ContentType::JPEG, services::image(&conn, &id))
}

/********************************************* Seasons *********************************************
***************************************************************************************************/

#[get("/seasons/id/<id>")]
pub fn season(conn: NiccDbConn, id: String) -> Result<Json<Vec<String>>, impl Error> {
    services::season(&conn, &id).map(Json)
}

#[get("/seasons/last")]
pub fn season_last(conn: NiccDbConn) -> Result<Json<Vec<String>>, impl Error> {
    services::season_last(&conn).map(Json)
}

#[get("/seasons/last/full")]
pub fn season_last_full(conn: NiccDbConn) -> Result<Json<Vec<String>>, impl Error> {
    services::season_last_full(&conn).map(Json)
}