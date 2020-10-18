use std::collections::{HashMap, HashSet};

use rocket_contrib::databases::r2d2_redis::redis::RedisResult;
use rocket_contrib::databases::redis::Commands;

use crate::compose;
use crate::entities::*;
use crate::responses::{ServiceError, ServiceResult};
use crate::NiccDbConn;
use crate::redis_keys::*;
use crate::views::UserView;

/********************************************* Users ***********************************************
***************************************************************************************************/

pub fn users(conn: &NiccDbConn) -> ServiceResult<HashSet<String>> {
    Ok(
        conn.smembers(compose!(USER, INDEX))?
    )
}

// TODO capire come propagare l'errore nelle closure invece che nasconderlo con ok()

pub fn user(conn: &NiccDbConn, id: &str) -> ServiceResult<User> {
    let map: HashMap<String, String> = conn.hgetall(compose!(USER, id))?;
    Ok(User::from(map).ok())
}

pub fn user_field(conn: &NiccDbConn, id: &String, field: &String) -> ServiceResult<String> {
    Ok(
        conn.hget(compose!(USER, id), field)?
    )
}

pub fn users_full(conn: &NiccDbConn) -> ServiceResult<HashSet<User>> {
    Ok(
        users(&conn)?
            .and_then(|id_set|
                id_set
                    .iter()
                    .filter_map(|id| user(conn, id).ok())
                    .collect()
            )
    )
}

fn user_by_username(conn: &NiccDbConn, username: &String) -> ServiceResult<User> {
    Ok(
        users(&conn)?
            .and_then(|id_set|
                id_set
                    .iter()
                    .find(|id| user_field(&conn, id, username).ok().flatten().is_some())
                    .and_then(|id|
                        user(&conn, id).ok()?
                    )
            )
    )
}

fn user_by_username_and_pw(conn: &NiccDbConn, auth: Credentials) -> ServiceResult<User> {
    Ok(
        user_by_username(conn, &auth.username)?
            .and_then(|usr|
                if usr.password.eq(&auth.password) { Some(usr) } else { None }
            )
    )
}

/********************************************* Auth ************************************************
***************************************************************************************************/

/**
* Returns an UserView of the authenticated user, or None.
*/
pub fn match_auth(conn: &NiccDbConn, auth: &Credentials) -> ServiceResult<UserView> {
    Ok(
        user_by_username(conn, &auth.username)?
           .and_then(|usr|
                         if usr.password.eq(&auth.password) { Some(UserView::from(usr)) } else {None})
    )
}


/********************************************* Queue ***********************************************
***************************************************************************************************/

pub fn queue(conn: &NiccDbConn) -> ServiceResult<Vec<String>> {
    Ok(conn.lrange(QUEUE, 0, -1)?)
}

pub fn queue_users(conn: &NiccDbConn) -> ServiceResult<Vec<User>> {
    println!("{:?} ", queue(conn)?.unwrap_or_default());
    println!("{:?}", user(conn, "1").ok());
    // println!("cane {:?}", queue(conn)?
    //     .unwrap_or_default()
    //     .iter()
    //     .filter_map(|_| Some(User{
    //         id: "".to_string(),
    //         nickname: "".to_string(),
    //         bio: "".to_string(),
    //         password: "".to_string()
    //     }))//|id| user(conn, id).ok().flatten())
    //     .collect::<Vec<User>>());
    Ok(
        Some(
            queue(conn)?
                .unwrap_or_default()
                .iter()
                .filter_map(|id| user(conn, id).ok().flatten())
                .collect()
        ))
}

/********************************************* Images **********************************************
***************************************************************************************************/

pub fn image(conn: &NiccDbConn, id: &str) -> ServiceResult<Vec<u8>> {
    Ok(conn.0.get(compose!(IMAGE, id))?)
}

/********************************************* Niccolgurs ******************************************
***************************************************************************************************/

pub fn niccolgur_members(conn: &NiccDbConn, id: &str) -> ServiceResult<Vec<String>> {
    Ok(conn.smembers(compose!(PARTICIPANTS, id))?)
}

pub fn niccolgur(conn: &NiccDbConn, id: &str) -> ServiceResult<Niccolgur> {
    let map: HashMap<String, String> = conn.hgetall(compose!(NICCOLGUR, id))?;
    let nic_transf = Niccolgur::from(&map);
    match nic_transf {
        Ok(mut nicc) =>
            Ok(
                niccolgur_members(conn, &id)?
                    .and_then(|memb_vec| {
                        nicc.members = memb_vec;
                        Some(nicc)
                    })
            ),
        Err(_) => Err(ServiceError {})
    }
}

/********************************************* Seasons *********************************************
***************************************************************************************************/

pub fn season(conn: &NiccDbConn, id: &str) -> ServiceResult<Vec<String>> {
    Ok(conn.lrange(compose!(SEASON, id), 0, -1)?)
}

pub fn seasons_count(conn: &NiccDbConn) -> ServiceResult<String> {
    Ok(
        conn.0.smembers::<String, HashSet<String>>(compose!(SEASON, INDEX))?
            .iter()
            .max()
            .map(String::to_owned)
    )
}

pub fn season_full(conn: &NiccDbConn, id: &str) -> ServiceResult<Vec<Niccolgur>> {
    Ok(
        Some(
            season(conn, id)?
                .unwrap_or_default()
                .iter()
                .filter_map(|nicc|
                    niccolgur(conn, nicc)
                        .ok()
                        .flatten())
                .collect()
        )
    )
}

pub fn season_last(conn: &NiccDbConn) -> ServiceResult<Vec<String>> {
    Ok(
        seasons_count(conn)?
            .and_then(|count|
                season(conn, &count).ok().flatten()
            ))
}

pub fn season_last_full(conn: &NiccDbConn) -> ServiceResult<Vec<Niccolgur>> {
    Ok(
        seasons_count(conn)?
            .and_then(|count|
                season_full(conn, &count).ok().flatten()
            ))
}