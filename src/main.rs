mod db;
mod model;
mod errors;
mod codec;

use mongodb::bson::{oid::ObjectId};

use log::{debug, error, info};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use std::env;
use dotenv::dotenv;
use tonic::{transport::Server, Request, Response, Status};
use crate::db::MongoDbClient;
use crate::model::{convert_order_status_to_pb, convert_to_order_status, OrderStatus};


use order_service::order_service::order_service_server::{
    OrderService, OrderServiceServer,
};

use order_service::order_service::{
     ListOrderByClientIdRequest,
     OrderListResponse,
     CreateOrderRequest,
     OrderResponse,
     SetOrderStatusRequest,
     SetCarRequest,
     Order,
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    info!("Starting Order service server");
    let addr = std::env::var("GRPC_SERVER_ADDRESS")?.parse()?;

    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI env var should be specified");
    let mongodb_client = MongoDbClient::new(mongodb_uri).await;
    let order_service = OrderServiceTest {
        mongo: mongodb_client,
    };

    let svc = OrderServiceServer::new(order_service);

    Server::builder().add_service(svc).serve(addr).await?;


    Ok(())
}


struct OrderServiceTest {
    mongo: MongoDbClient,
}

#[tonic::async_trait]
impl OrderService for OrderServiceTest {
    async fn list_order_by_client_id(&self,
                                     request: tonic::Request<ListOrderByClientIdRequest>)
                                     -> Result<tonic::Response<OrderListResponse>, Status>{
        debug!("list_order_by_client_id a request: {:?}", request);

        let client_id = request.into_inner().client_id;

        debug!("client_id: {}", client_id);

        //get orders from mongo
        let orders =
            self.mongo.list_orders_by_client_id(client_id).await;

        let mut orders_pb = Vec::new();

        for value in orders {
            println!("value: {:?}", value);
        }

        Ok(Response::new(OrderListResponse {
            orders: orders_pb,
        }))

    }

    async fn create_order(&self,
                          request: tonic::Request<CreateOrderRequest>,)
                          -> Result<tonic::Response<OrderResponse>, Status>{
        debug!("create_order a request: {:?}", request);

        let proto_req = request.into_inner();

        let to = proto_req.to;
        let from = proto_req.from;
        let client_id = proto_req.client_id;

        debug!("to: {}, from: {}, client_id: {}", to, from, client_id);

        let order = self.mongo.create_order(from,to,client_id)
            .await;

        let res = order.as_ref().unwrap();

        let x: Option<Order> = Some(Order{
            id: res.id.unwrap().to_hex(),
            from: res.from,
            to: res.to,
            client_id: res.client_id,
            car_id: res.car_id.to_string(),
            status: convert_order_status_to_pb(res.status),
        });
        debug!("GRPC: {:?}", x);

        Ok(Response::new( OrderResponse {
            order: x,
        }))
    }

    async fn set_order_status(&self,
                              request: tonic::Request<SetOrderStatusRequest>,)
                              -> Result<tonic::Response<()>, Status>{
        debug!("set_order_status a request: {:?}", request);

        let proto_req = request.into_inner();

        let order_id = proto_req.order_id;

        //i32 representation
        let status = proto_req.status;

        debug!("set_order_status order_id: {}, {}", order_id, status);


        Ok(Response::new(self.mongo.order_status_set(order_id,
                                                     convert_to_order_status(status)).await))
    }

    async fn set_car(&self,
                       request: tonic::Request<SetCarRequest>,)
                       -> Result<tonic::Response<()>, Status>{
          debug!("set_car a request: {:?}", request);

          let proto_req = request.into_inner();
          let car_id = proto_req.car_id;
          let order_id = proto_req.order_id;

          debug!("set_car {}, {}", car_id, order_id);

         Ok(Response::new(self.mongo.order_car_set(order_id, car_id).await))
    }
}