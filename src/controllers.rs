use std::collections::HashSet;
use rocket_contrib::json::Json;
use crate::entities::*;
use crate::NiccDbConn;
use crate::services::{self};

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello cane laido!"
}

/********************************************* Users ***********************************************
***************************************************************************************************/


#[get("/users")]
pub fn users(conn: NiccDbConn) -> Json<HashSet<String>> {
    Json(services::users(&conn))
}

#[get("/users/full")]
pub fn users_full(conn: NiccDbConn) -> Json<HashSet<User>> {
    Json(services::users(&conn).iter().filter_map(|id| services::user(&conn, id)).collect())
}

#[get("/users/id/<id>")]
pub fn user(conn: NiccDbConn, id: String) -> Json<Option<User>> {
    Json(services::user(&conn, &id))
}