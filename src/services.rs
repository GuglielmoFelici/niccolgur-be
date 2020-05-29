use crate::NiccDbConn;
use std::collections::{HashSet, HashMap};
use crate::entities::*;
use crate::redis_keys::*;
use crate::compose;
use rocket_contrib::databases::redis::Commands;

// TODO error checking in redis calls

/********************************************* Users ***********************************************
***************************************************************************************************/

pub fn users(conn: &NiccDbConn) -> HashSet<String> {
    match conn.smembers(compose!(USER, INDEX)) {
        Ok(set) => set,
        Err(_) => HashSet::new(),
    }
}

pub fn user(conn: &NiccDbConn, id: &str) -> Option<User> {
    let user_map: HashMap<String, String> = match conn.hgetall(compose!(USER, id)) {
        Ok(map) => map,
        Err(_) => return None,
    };
    User::from_map(&user_map)
}

pub fn users_full(conn: &NiccDbConn) -> HashSet<User> {
    users(&conn).iter().filter_map(|id| user(&conn, id)).collect()
}

/********************************************* Queue ***********************************************
***************************************************************************************************/

pub fn queue(conn: &NiccDbConn) -> Vec<String> {
    match conn.lrange(QUEUE, 0, -1) {
        Ok(vec) => vec,
        Err(_) => Vec::new(),
    }
}

pub fn users_queue(conn: &NiccDbConn) -> Vec<User> {
    queue(conn).iter().filter_map(|id| user(&conn, id)).collect()
}

/********************************************* Images **********************************************
***************************************************************************************************/

pub fn image(conn: &NiccDbConn, id: &str) -> Option<Vec<u8>> {
    match conn.get(compose!(IMAGE, id)) {
        Ok(img) => img,
        Err(_e) => None,
    }
}