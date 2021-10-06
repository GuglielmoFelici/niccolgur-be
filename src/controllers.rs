use std::collections::HashSet;
use std::path::PathBuf;

//use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
//use jsonwebtoken::errors::Error;
use rocket::{Response};
use rocket::http::ContentType;
//use rocket::http::hyper::header::Bearer;
use rocket::http::Status;
use rocket::response::{Content};
use rocket_contrib::json::Json;

//use crate::auth::{Claims, generate_token, TokenVerifier};
use crate::entities::*;
use crate::NiccDbConn;
use crate::responses::{ControllerError, ControllerResult, ControllerStreamResult};
use crate::services::{self};
use crate::views::{ UserView};

#[get("/")]
pub fn hello() -> &'static str {
    "Hello cane laido!"
}



// TODO capire come rispondere bene
#[options("/<path..>")]
pub fn pre_flight(path: PathBuf) -> Response<'static> {
    Response::build().raw_header("Access-Control-Allow-Origin", "*").finalize()
}



/********************************************* Queue ***********************************************
***************************************************************************************************/

#[get("/queue")]
pub fn queue(conn: NiccDbConn) -> ControllerResult<Vec<String>> {
    Ok(Json(
        services::queue(&conn)?
            .unwrap_or_default()
    ))
}

#[get("/queue/full")]
pub fn queue_users(conn: NiccDbConn) -> ControllerResult<Vec<UserView>> {
    Ok(
        Json(
            services::queue_users(&conn)?
                .unwrap_or_default()
                .iter()
                .map(|usr| UserView::from(usr))
                .collect()
        )
    )
}

/********************************************* Users ***********************************************
***************************************************************************************************/


#[get("/users")]
pub fn users(conn: NiccDbConn) -> ControllerResult<HashSet<String>> {
    Ok(
        Json(services::users(&conn)?.unwrap_or_default())
    )
}

#[get("/users/full")]
pub fn users_full(conn: NiccDbConn) -> ControllerResult<HashSet<UserView>> {
    Ok(
        Json(
            services::users_full(&conn)?
                .unwrap_or_default()
                .iter()
                .map(|usr| UserView::from(usr))
                .collect()
        )
    )
}

#[get("/users/id/<id>")]
pub fn user(conn: NiccDbConn, id: String) -> ControllerResult<UserView> {
    Ok(
        Json(UserView::from(
            services::user(&conn, &id)?
                .ok_or(ControllerError(Status::NotFound))?
        ))
    )
}

/********************************************* Auth ************************************************
***************************************************************************************************/

//#[get("/protected")]
//pub fn hello_protected(guard: TokenVerifier) -> &'static str {
//    "Hello cane laido loggato!"
//}

//#[post("/login", data = "<credentials>")]
//pub fn login<'r>(conn: NiccDbConn, credentials: Json<Credentials>) -> ControllerResult<TokenView> { //TODO return value
//    services::match_auth(&conn, &credentials)?
//        .and_then(|view|
//            Some(
//                generate_token(&view.id).ok()?)
//            )
//        .ok_or(ControllerError(Status::Unauthorized))
//}

//#[get("/logindata/<token>")]
//pub fn token_data(token: String) -> Result<String, Error> {
//    Ok(jsonwebtoken::decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::new(Algorithm::HS256) )?.claims.sub)
//}


/********************************************* Images **********************************************
***************************************************************************************************/

#[get("/images/id/<id>")]
pub fn image(conn: NiccDbConn, id: String) -> ControllerStreamResult {
    Content(ContentType::JPEG, Ok(services::image(&conn, &id)?.unwrap_or_default())).1
}

/********************************************* Seasons *********************************************
***************************************************************************************************/

#[get("/seasons/id/<id>")]
pub fn season(conn: NiccDbConn, id: String) -> ControllerResult<Vec<String>> {
    Ok(
        Json(services::season(&conn, &id)?.unwrap_or_default())
    )
}

#[get("/seasons/id/<id>/full")]
pub fn season_full(conn: NiccDbConn, id: String) -> ControllerResult<Vec<Niccolgur>> {
    Ok(
        Json(services::season_full(&conn, &id)?.unwrap_or_default())
    )
}

#[get("/seasons/count")]
pub fn seasons_count(conn: NiccDbConn) -> ControllerResult<String> {
    Ok(
        Json(services::seasons_count(&conn)?.unwrap_or_default())
    )
}

#[get("/seasons/last")]
pub fn season_last(conn: NiccDbConn) -> ControllerResult<Vec<String>> {
    Ok(
        Json(services::season_last(&conn)?.unwrap_or_default())
    )
}

#[get("/seasons/last/full")]
pub fn season_last_full(conn: NiccDbConn) -> ControllerResult<Vec<Niccolgur>> {
    Ok(
        Json(services::season_last_full(&conn)?.unwrap_or_default())
    )
}
