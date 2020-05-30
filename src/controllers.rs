use std::collections::HashSet;
use rocket_contrib::json::Json;
use crate::entities::*;
use crate::NiccDbConn;
use crate::services::{self};
use rocket::response::Content;
use rocket::http::ContentType;
use rocket_contrib::databases::r2d2_redis::redis::RedisError;

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
pub fn queue_users(conn: NiccDbConn) -> Result<Json<Vec<User>>, RedisError> {
    services::queue_users(&conn).map(Json)
}

/********************************************* Users ***********************************************
***************************************************************************************************/


#[get("/users")]
pub fn users(conn: NiccDbConn) -> Result<Json<HashSet<String>>, RedisError> {
    services::users(&conn).map(Json)
}

#[get("/users/full")]
pub fn users_full(conn: NiccDbConn) -> Result<Json<HashSet<User>>, RedisError> {
    services::users_full(&conn).map(Json)
}

#[get("/users/id/<id>")]
pub fn user(conn: NiccDbConn, id: String) -> Result<Json<User>, RedisError> {
    services::user(&conn, &id).map(Json)
}


/********************************************* Images **********************************************
***************************************************************************************************/

#[get("/images/id/<id>")]
pub fn image(conn: NiccDbConn, id: String) -> Content<Result<Vec<u8>, RedisError>> {
    Content(ContentType::JPEG, services::image(&conn, &id))
}

/********************************************* Seasons *********************************************
***************************************************************************************************/

#[get("/seasons/last")]
pub fn season_last(conn: NiccDbConn) -> Result<Json<Vec<String>>, impl Error> {
    .map(Json)
}

#[get("/seasons/last/full")]
pub fn season_last_full(conn: NiccDbConn) -> Content<Option<Vec<u8>>> {
    Content(ContentType::JPEG, services::image(&conn, &id))
}