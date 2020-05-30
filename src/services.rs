use std::collections::{HashMap, HashSet};
use std::error::Error;

use rocket_contrib::databases::r2d2_redis::redis::{RedisError, RedisResult};
use rocket_contrib::databases::redis::Commands;

use crate::compose;
use crate::entities::*;
use crate::NiccDbConn;
use crate::redis_keys::*;
use std::io::ErrorKind;

// TODO error checking in redis calls

/********************************************* Users ***********************************************
***************************************************************************************************/

pub fn users(conn: &NiccDbConn) -> RedisResult<HashSet<String>> {
    conn.smembers(compose!(USER, INDEX))
}

pub fn user(conn: &NiccDbConn, id: &str) -> RedisResult<User> {
    let map = conn.0.hgetall(compose!(USER, id))?;
    match User::from_map(&map) {
        Some(User) => Ok(User),
        None =>
    }
}

pub fn users_full(conn: &NiccDbConn) -> Result<HashSet<User>, impl Error> {
    Ok(users(&conn)?.iter().filter_map(|id| user(conn, id).ok()).collect())
}

/********************************************* Queue ***********************************************
***************************************************************************************************/

pub fn queue(conn: &NiccDbConn) -> Result<Vec<String>, impl Error> {
    conn.lrange(QUEUE, 0, -1)
}

pub fn queue_users(conn: &NiccDbConn) -> Result<Vec<User>, impl Error> {
    Ok(queue(conn)?.iter().filter_map(|id| user(conn, id).ok()).collect())
}

/********************************************* Images **********************************************
***************************************************************************************************/

pub fn image(conn: &NiccDbConn, id: &str) -> Result<Vec<u8>, impl Error> {
    conn.get(compose!(IMAGE, id))
}

/********************************************* Niccolgurs ******************************************
***************************************************************************************************/

pub fn niccolgur_full(conn: &NiccDbConn, id: &str) -> Result<Niccolgur, impl Error> {
    // TODO
}

/********************************************* Seasons *********************************************
***************************************************************************************************/

pub fn season(conn: &NiccDbConn, id: &str) -> Result<Vec<String>, impl Error> {
    conn.lrange(compose!(SEASON, id), 0, -1)
}

pub fn season_full(conn: &NiccDbConn, id: &str) -> Result<Vec<Niccolgur>, impl Error> {
    season(conn, id)?.iter().filter_map(|nicc| niccolgur_full(conn, id).ok()).collect()
}

pub fn season_last(conn: &NiccDbConn) -> Result<Vec<String>, impl Error> {
    let idx: HashSet<String> = conn.0.smembers(compose!(SEASON, INDEX))?;
    season(conn, idx.iter().max().ok_or(0)?)
}

pub fn season_last_full(conn: &NiccDbConn) -> Result<Vec<Niccolgur>, impl Error> {
    let idx: HashSet<String> = conn.0.smembers(compose!(SEASON, INDEX))?;
    season_full(conn, idx.iter().max().ok_or(0)?)
}