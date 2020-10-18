use std::fmt;
use std::error::Error;
use rocket_contrib::databases::r2d2_redis::redis::RedisError;
use rocket_contrib::json::{Json, JsonValue};
use rocket::response::Responder;
use rocket::{Request, response};
use rocket::http::{Status, ContentType};
use rocket::http::hyper::header::Bearer;

/********************************************* Entities ********************************************
***************************************************************************************************/

#[derive(Debug)]
pub struct TransformError;

impl fmt::Display for TransformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "There was an error while processing the entity.")
    }
}

impl Error for TransformError {}

pub type TransformResult<T> = Result<T, TransformError>;

/********************************************* Controllers *****************************************
***************************************************************************************************/

#[derive(Debug)]
pub struct ControllerError(pub Status);

impl fmt::Display for ControllerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The controller returned an expected error. {:?}", self.0)
    }
}

impl<'r> Responder<'r> for ControllerError {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Err(self.0)
    }
}

impl Error for ControllerError {}

impl From<ServiceError> for ControllerError {
    fn from(_: ServiceError) -> Self {
        ControllerError(Status::InternalServerError)
    }
}

pub type ControllerResult<T> = Result<Json<T>, ControllerError>; // TODO rimuovere json
pub type ControllerStreamResult = Result<Vec<u8>, ControllerError>;

// impl <T> From<Json<T>> for ControllerResult<T> {
//     fn from(json: Json<T>) -> Self {
//         Ok(json)
//     }
// }

/********************************************* Services ********************************************
***************************************************************************************************/

#[derive(Debug)]
pub struct ServiceError;

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The Redis service returned an expected error.")
    }
}

impl Error for ServiceError {}

impl From<RedisError> for ServiceError {
    fn from(_: RedisError) -> Self {
        ServiceError {
            // TODO
        }
    }
}

impl From<TransformError> for ServiceError {
    fn from(_: TransformError) -> Self {
        ServiceError {
            // TODO
        }
    }
}

pub type ServiceResult<T> = Result<Option<T>, ServiceError>;
