use std::collections::HashSet;

use rocket::http::ContentType;
use rocket::response::Content;
use rocket_contrib::json::Json;

use crate::entities::*;
use crate::NiccDbConn;
use crate::services::{self};
use crate::errors::{ControllerResult, ControllerStreamResult};

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello cane laido!"
}

/********************************************* Queue ***********************************************
***************************************************************************************************/

#[get("/queue")]
pub fn queue(conn: NiccDbConn) -> ControllerResult<Vec<String>> {
    Ok(Json(services::queue(&conn)?))
}

#[get("/queue/full")]
pub fn queue_users(conn: NiccDbConn) -> ControllerResult<Vec<User>> {
    services::queue_users(&conn).map(Json)
}

/********************************************* Users ***********************************************
***************************************************************************************************/


#[get("/users")]
pub fn users(conn: NiccDbConn) -> ControllerResult<HashSet<String>> {
    services::users(&conn).map(Json)
}

#[get("/users/full")]
pub fn users_full(conn: NiccDbConn) -> ControllerResult<HashSet<User>> {
    services::users_full(&conn).map(Json)
}

#[get("/users/id/<id>")]
pub fn user(conn: NiccDbConn, id: String) -> ControllerResult<User> { services::user(&conn, &id).map(Json)
}


/********************************************* Images **********************************************
***************************************************************************************************/

#[get("/images/id/<id>")]
pub fn image(conn: NiccDbConn, id: String) -> Content<ControllerStreamResult> {
    Content(ContentType::JPEG, services::image(&conn, &id))
}

/********************************************* Seasons *********************************************
***************************************************************************************************/

#[get("/seasons/id/<id>")]
pub fn season(conn: NiccDbConn, id: String) -> ControllerResult<Vec<String>> {
    services::season(&conn, &id).map(Json)
}

#[get("/seasons/last")]
pub fn season_last(conn: NiccDbConn) -> ControllerResult<Vec<String>> {
    services::season_last(&conn).map(Json)
}

#[get("/seasons/last/full")]
pub fn season_last_full(conn: NiccDbConn) -> ControllerResult<Vec<Niccolgur>> {
    services::season_last_full(&conn).map(Json)
}