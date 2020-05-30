use crate::NiccDbConn;
use std::collections::{HashSet, HashMap};
use crate::entities::*;
use crate::redis_keys::*;
use crate::compose;
use rocket_contrib::databases::redis::Commands;
use rocket_contrib::databases::r2d2_redis::redis::RedisError;
use std::error::Error;

// TODO error checking in redis calls

/********************************************* Users ***********************************************
***************************************************************************************************/

pub fn users(conn: &NiccDbConn) -> Result<HashSet<String>, impl Error> {
    conn.smembers(compose!(USER, INDEX))
}

pub fn user(conn: &NiccDbConn, id: &str) -> Result<User, impl Error> {
    let user_map: HashMap<String, String> = conn.hgetall(compose!(USER, id))?;
    User::from_map(&user_map)
}

pub fn users_full(conn: &NiccDbConn) -> Result<HashSet<User>, RedisError> {
    users(&conn)?.iter().filter_map(|id| user(conn, id).ok()).collect()
}

/********************************************* Queue ***********************************************
***************************************************************************************************/

pub fn queue(conn: &NiccDbConn) -> Result<Vec<String>, impl Error> {
    conn.lrange(QUEUE, 0, -1)
}

pub fn queue_users(conn: &NiccDbConn) -> Result<Vec<User>, impl Error> {
    queue(conn)?.iter().filter_map(|id| user(conn, id).ok()).collect()
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
    // TODO
}

pub fn season_last_full(conn: &NiccDbConn) -> Result<Vec<Niccolgur>, impl Error> {
    // TODO
}