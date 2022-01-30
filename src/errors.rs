use derive_more::{Display, Error};
use log::error;
use serde::Serialize;

#[derive(Debug, Display, Error)]
pub enum CustomError {
    #[display(fmt = message)]
    NotFound {
        message: String,
    },
    #[display(fmt = message)]
    MongoDbError {
        message: String,
    },
}

impl CustomError {
    fn name(&self) -> String {
        let name = match self {
            Self::NotFound { message: _ } => "Resource not found",
            Self::MongoDbError { message: _ } => "MongoDB error",
        };

        String::from(name)
    }
}

impl From<mongodb::error::Error> for CustomError {
    fn from(source: mongodb::error::Error) -> Self {
        Self::MongoDbError {
            message: source.to_string(),
        }
    }
}

impl From<mongodb::bson::de::Error> for CustomError {
    fn from(source: mongodb::bson::de::Error) -> Self {
        Self::MongoDbError {
            message: source.to_string(),
        }
    }
}

impl From<mongodb::bson::ser::Error> for CustomError {
    fn from(source: mongodb::bson::ser::Error) -> Self {
        Self::MongoDbError {
            message: source.to_string(),
        }
    }
}

impl From<mongodb::bson::oid::Error> for CustomError {
    fn from(source: mongodb::bson::oid::Error) -> Self {
        Self::NotFound {
            message: source.to_string(),
        }
    }
}