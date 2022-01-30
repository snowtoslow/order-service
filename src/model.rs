use serde::{Deserialize, Serialize};
use mongodb::bson::{self, doc, oid::ObjectId, Document};
use std::str::FromStr;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderDB {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub from: u64,
    pub to: u64,
    pub r#status: OrderStatus,
    pub client_id: u64,
    pub car_id: String,
}

impl From<&OrderDB> for Document {
    fn from(source: &OrderDB) -> Self {
        bson::to_document(source).expect("Can't convert a planet to Document")
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum OrderStatus {
    UNKNOWN,
    WAITING,
    IN_PROGRESS,
    DONE,
}

pub fn convert_to_order_status(status: i32) -> OrderStatus {
    match status {
        1 => OrderStatus::WAITING,
        2 => OrderStatus::IN_PROGRESS,
        3 => OrderStatus::DONE,
        //todo: to change
        _ => OrderStatus::UNKNOWN,
    }
}

pub fn convert_order_status_to_pb(status: OrderStatus) -> i32{
    match status {
        OrderStatus::WAITING => 1,
        OrderStatus::IN_PROGRESS => 2,
        OrderStatus::DONE => 3,
        OrderStatus::UNKNOWN => 0
    }
}


impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}