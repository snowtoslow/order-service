use mongodb::bson::{doc, oid::ObjectId};
use mongodb::{Client, Collection};
use tokio_stream::StreamExt;
use std::str::FromStr;

use crate::errors::CustomError;
use crate::errors::CustomError::NotFound;

use crate::model::{OrderDB, OrderStatus};



const DB_NAME: &str = "order-service-database";
const COLLECTION_NAME: &str = "orders";

#[derive(Clone, Debug)]
pub struct MongoDbClient {
    client: Client,
}

impl MongoDbClient {
    pub async fn new(mongodb_uri: String) -> Self {
        let mongodb_client = Client::with_uri_str(mongodb_uri)
            .await
            .expect("Failed to create MongoDB client");

        MongoDbClient {
            client: mongodb_client,
        }
    }

    pub async fn list_orders_by_client_id(&self,client_id: String) -> Result<Vec<OrderDB>, CustomError>{
        let filter = doc! { "client_id": &client_id };

        let mut orders = self.get_orders_collection().find(filter, None).await?;

        let mut result: Vec<OrderDB> = Vec::new();

        while let Some(order) = orders.next().await {
            result.push(order?);
        }

        Ok(result)
    }

    pub async fn create_order(&self, from: u64, to: u64, client_id: u64) -> Result<OrderDB, CustomError> {
        let collection = self.get_orders_collection();

        let order_to_create = OrderDB{
            id: None,
            from,
            to,
            status: OrderStatus::WAITING,
            client_id,
            car_id: "".to_string()
        };

        let insert_result = collection.insert_one(order_to_create, None).await?;
        let filter = doc! { "_id": &insert_result.inserted_id };
        collection.find_one(filter, None).await?.ok_or(NotFound {
            message: String::from("Can't find a created order"),
        })
    }

    pub async fn order_status_set(&self, order_id: String, status: OrderStatus){
        let collection = self.get_orders_collection();

        let filter = doc! {"_id": ObjectId::from_str(&order_id).unwrap()};

        let update = doc! {"$set": { "status": format!(r#"{:?}"#, status) }};

        collection.update_one(filter, update, None).await.unwrap();
    }

    pub async fn order_car_set(&self, order_id: String, car_id: String){
        let collection = self.get_orders_collection();

        let filter = doc! { "_id": ObjectId::from_str(&order_id).unwrap()};

        let update = doc!{"$set": {"car_id": car_id}};

        collection.update_one(filter, update, None).await.unwrap();
    }

    fn get_orders_collection(&self) -> Collection<OrderDB> {
        self.client
            .database(DB_NAME)
            .collection::<OrderDB>(COLLECTION_NAME)
    }

    fn test(a: Vec<u8>){

    }
}

