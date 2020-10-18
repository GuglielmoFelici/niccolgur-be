/************************************ Base keys ***************************************************/

pub const USER: &str = "user";
pub const NICCOLGUR: &str = "niccolgur";
pub const PARTICIPANTS: &str = "participants";
pub const SEASON: &str = "season";
pub const IMAGE: &str = "image";
pub const QUEUE: &str = "queue";

/************************************ Field keys **************************************************/

pub const USER_PW: &str = "password";

/************************************ Secondary keys **********************************************/

pub const DELIM: &str = ":";
/** x:index contains an id for every x:id */
pub const INDEX: &str = "index";
/** x:id contains the largest id used in x:id */
pub const ID: &str = "id";

#[macro_export]
macro_rules! compose {
    ( $x:expr, $y:expr ) => { format!("{}{}{}", $x, DELIM, $y) };
}