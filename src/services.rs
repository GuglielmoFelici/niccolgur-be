use crate::NiccDbConn;
use std::collections::{HashSet, HashMap};
use crate::entities::*;
use crate::redis_keys::*;
use crate::compose;
use rocket_contrib::databases::redis::Commands;

// TODO error checking in redis calls

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