use std::collections::HashSet;
use rocket_contrib::json::Json;
use crate::entities::*;
use crate::NiccDbConn;
use crate::services::{self};
use rocket::response::Content;
use rocket::http::ContentType;

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello cane laido!"
}

/********************************************* Queue ***********************************************
***************************************************************************************************/

#[get("/queue")]
pub fn queue(conn: &NiccDbConn) -> Json<Vec<String>> {
    Json(services::queue(&conn))
}

#[get("/queue/full")]
pub fn users_queue(conn: NiccDbConn) -> Json<Vec<User>> {
    Json(services::users_queue(&conn))
}

/********************************************* Users ***********************************************
***************************************************************************************************/


#[get("/users")]
pub fn users(conn: NiccDbConn) -> Json<HashSet<String>> {
    Json(services::users(&conn))
}

#[get("/users/full")]
pub fn users_full(conn: NiccDbConn) -> Json<HashSet<User>> {
    Json(services::users_full(&conn))
}

#[get("/users/id/<id>")]
pub fn user(conn: NiccDbConn, id: String) -> Json<Option<User>> {
    Json(services::user(&conn, &id))
}


/********************************************* Images **********************************************
***************************************************************************************************/

#[get("/images/id/<id>")]
pub fn image(conn: NiccDbConn, id: String) -> Content<Option<Vec<u8>>> {
    Content(ContentType::JPEG, services::image(&conn, &id))
}