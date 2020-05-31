use std::collections::HashSet;

use rocket_contrib::databases::redis::Commands;

use crate::compose;
use crate::entities::*;
use crate::NiccDbConn;
use crate::redis_keys::*;
use crate::errors::{ServiceError, ServiceResult};

/********************************************* Users ***********************************************
***************************************************************************************************/

pub fn users(conn: &NiccDbConn) -> ServiceResult<HashSet<String>> {
    Ok(conn.smembers(compose!(USER, INDEX))?)
}

pub fn user(conn: &NiccDbConn, id: &str) -> ServiceResult<User> {
    let map = conn.hgetall(compose!(USER, id))?;
    Ok(User::from_map(&map)?)
}

pub fn users_full(conn: &NiccDbConn) -> ServiceResult<HashSet<User>> {
    Ok(users(&conn)?.iter().filter_map(|id| user(conn, id).ok()).collect())
}

/********************************************* Queue ***********************************************
***************************************************************************************************/

pub fn queue(conn: &NiccDbConn) -> ServiceResult<Vec<String>> {
    Ok(conn.lrange(QUEUE, 0, -1)?)
}

pub fn queue_users(conn: &NiccDbConn) -> ServiceResult<Vec<User>> {
    Ok(queue(conn)?.iter().filter_map(|id| user(conn, id).ok()).collect())
}

/********************************************* Images **********************************************
***************************************************************************************************/

pub fn image(conn: &NiccDbConn, id: &str) -> ServiceResult<Vec<u8>> {
    Ok(conn.get(compose!(IMAGE, id))?)
}

/********************************************* Niccolgurs ******************************************
***************************************************************************************************/

pub fn niccolgur_members(conn: &NiccDbConn, id: &str) -> ServiceResult<Vec<String>> {
    Ok(conn.0.lrange(compose!(PARTICIPANTS, id), 0, -1)?)
}

pub fn niccolgur(conn: &NiccDbConn, id: &str) -> ServiceResult<Niccolgur> {
    let map = conn.hgetall(compose!(NICCOLGUR, id))?;
    let mut nicc = Niccolgur::from_map(&map)?;
    nicc.members = niccolgur_members(conn, &id)?;
    Ok(nicc)
}

/********************************************* Seasons *********************************************
***************************************************************************************************/

pub fn season(conn: &NiccDbConn, id: &str) -> ServiceResult<Vec<String>> {
    Ok(conn.lrange(compose!(SEASON, id), 0, -1)?)
}

pub fn season_full(conn: &NiccDbConn, id: &str) -> ServiceResult<Vec<Niccolgur>> {
    Ok(season(conn, id)?.iter().filter_map(|nicc| niccolgur(conn, nicc).ok()).collect())
}

pub fn season_last(conn: &NiccDbConn) -> ServiceResult<Vec<String>> {
    let idx: HashSet<String> = conn.smembers(compose!(SEASON, INDEX))?;
    season(conn, idx.iter().max().ok_or(ServiceError)?)
}

pub fn season_last_full(conn: &NiccDbConn) -> ServiceResult<Vec<Niccolgur>> {
    let idx: HashSet<String> = conn.smembers(compose!(SEASON, INDEX))?;
    season_full(conn, idx.iter().max().ok_or(ServiceError)?)
}