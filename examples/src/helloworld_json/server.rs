use tonic::{transport::Server, Request, Response, Status};

use hello_world_json::json_greeter_server::{JsonGreeter, JsonGreeterServer};
use hello_world_json::{HelloReply, HelloRequest};

pub mod hello_world_json {
    tonic::include_proto!("helloworld_json");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl JsonGreeter for MyGreeter {
    async fn method1(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = hello_world_json::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(JsonGreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
