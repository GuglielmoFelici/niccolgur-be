use std::fmt;
use std::error::Error;
use rocket_contrib::databases::r2d2_redis::redis::RedisError;
use rocket_contrib::json::Json;

/********************************************* Entities ********************************************
***************************************************************************************************/

#[derive(Debug)]
pub struct EntityError;

impl fmt::Display for EntityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "There was an error while processing the entity.")
    }
}

impl Error for EntityError {}

pub type EntityResult<T> = Result<T, EntityError>;

/********************************************* Controllers *****************************************
***************************************************************************************************/

// TODO cambiare errore?

pub type ControllerResult<T> = Result<Json<T>, ServiceError>;
pub type ControllerStreamResult = Result<Vec<u8>, ServiceError>;

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

impl From<EntityError> for ServiceError {
    fn from(_: EntityError) -> Self {
        ServiceError {
            // TODO
        }
    }
}

pub type ServiceResult<T> = Result<T, ServiceError>;